package dev.rustchunk.app

import android.content.Context
import android.system.Os
import org.apache.commons.compress.archivers.tar.TarArchiveInputStream
import org.apache.commons.compress.compressors.xz.XZCompressorInputStream
import java.io.BufferedInputStream
import java.io.File
import java.io.FileOutputStream
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.util.concurrent.TimeUnit

/**
 * Installs the ARM64 Java 17 payload from the already-installed ZalithLauncher2 APK.
 *
 * The runtime archives are read through the other app's public APK assets, not from its
 * private /data/user/0 directory. No runtime binary is bundled into RustChunk or downloaded.
 */
object JavaRuntimeInstaller {
    private const val ASSET_ROOT = "runtimes/jre-17"
    private const val RUNTIME_NAME = "Internal-17"
    private const val MAX_TAR_ENTRIES = 40000
    private const val MAX_EXPANDED_BYTES = 512L * 1024L * 1024L
    private val sourcePackages = listOf(
        "com.movtery.zalithlauncher.v2.debug",
        "com.movtery.zalithlauncher.v2",
        "com.movtery.zalithlauncher"
    )

    private data class Source(val packageName: String, val context: Context)

    private class ExtractionBudget {
        var entries = 0
        var bytes = 0L

        fun addEntry(size: Long) {
            entries++
            if (entries > MAX_TAR_ENTRIES) {
                throw IllegalStateException("Runtime archive contains too many entries.")
            }
            if (size > 0) {
                bytes += size
                if (bytes > MAX_EXPANDED_BYTES) {
                    throw IllegalStateException("Runtime archive expands beyond the safety limit.")
                }
            }
        }
    }

    fun installFromInstalledZalith(
        context: Context,
        destinationParent: File,
        onProgress: (String) -> Unit
    ): String {
        if (!android.os.Build.SUPPORTED_ABIS.contains("arm64-v8a")) {
            throw IllegalStateException("This RustChunk build currently supports ARM64 only.")
        }

        val source = findZalithSource(context)
        onProgress("Found Zalith package: " + source.packageName + "\nReading its Java 17 runtime assets...")
        val version = source.context.assets.open("$ASSET_ROOT/version").bufferedReader().use { it.readText().trim() }
        if (version.isBlank()) {
            throw IllegalStateException("The installed Zalith Java 17 package has an empty version marker.")
        }

        if (!destinationParent.exists() && !destinationParent.mkdirs()) {
            throw IllegalStateException("Cannot create runtime directory: " + destinationParent.absolutePath)
        }
        val availableBytes = android.os.StatFs(destinationParent.absolutePath).availableBytes
        if (availableBytes < 250L * 1024L * 1024L) {
            throw IllegalStateException("At least 250 MB of free storage is required to install Java 17.")
        }

        val staging = File(destinationParent, ".$RUNTIME_NAME.installing")
        val destination = File(destinationParent, RUNTIME_NAME)
        val backup = File(destinationParent, ".$RUNTIME_NAME.previous")
        if (staging.exists() && !staging.deleteRecursively()) {
            throw IllegalStateException("Cannot remove an incomplete previous runtime installation.")
        }
        if (!staging.mkdirs()) {
            throw IllegalStateException("Cannot create temporary runtime directory: " + staging.absolutePath)
        }

        val budget = ExtractionBudget()
        try {
            onProgress("Extracting Java 17 common files from Zalith...")
            source.context.assets.open("$ASSET_ROOT/universal.tar.xz").use { input ->
                extractTarXz(input, staging, budget, "Common Java files", onProgress)
            }

            onProgress("Extracting Java 17 ARM64 files from Zalith...")
            source.context.assets.open("$ASSET_ROOT/bin-arm64.tar.xz").use { input ->
                extractTarXz(input, staging, budget, "ARM64 Java files", onProgress)
            }

            File(staging, "version").writeText(version)
            enableBinExecutables(staging)

            val java = File(staging, "bin/java")
            if (!java.isFile) {
                throw IllegalStateException("The extracted Zalith runtime does not contain bin/java.")
            }
            java.setExecutable(true, true)
            val libjvm = staging.walkTopDown().maxDepth(9).firstOrNull {
                it.isFile && it.name == "libjvm.so"
            }
            if (libjvm == null) {
                throw IllegalStateException("The extracted Java 17 runtime does not contain libjvm.so.")
            }

            if (backup.exists() && !backup.deleteRecursively()) {
                throw IllegalStateException("Cannot remove the previous runtime backup.")
            }
            var movedOldRuntime = false
            if (destination.exists()) {
                if (!destination.renameTo(backup)) {
                    throw IllegalStateException("Cannot move the previous Java runtime out of the way.")
                }
                movedOldRuntime = true
            }
            if (!staging.renameTo(destination)) {
                if (movedOldRuntime) backup.renameTo(destination)
                throw IllegalStateException("Cannot finalize the Java 17 runtime installation.")
            }
            if (backup.exists()) backup.deleteRecursively()

            val javaTest = testJava(destination, context)
            val report = buildString {
                appendLine("RustChunk Java 17 runtime installation")
                appendLine("Source package: " + source.packageName)
                appendLine("Zalith runtime asset version: " + version)
                appendLine("Runtime directory: " + destination.absolutePath)
                appendLine("Java executable: " + File(destination, "bin/java").absolutePath)
                appendLine("JVM library: " + File(libjvm.absolutePath.replace(staging.absolutePath, destination.absolutePath)).absolutePath)
                appendLine("Archive entries extracted: " + budget.entries)
                appendLine("Expanded archive bytes: " + budget.bytes)
                appendLine()
                append(javaTest)
                appendLine()
                appendLine("This verifies Java startup only; Forge and Minecraft have not been launched.")
            }

            val reportFile = File(context.filesDir, "runtime/java17-install-report.txt")
            if (!reportFile.parentFile!!.exists()) reportFile.parentFile!!.mkdirs()
            reportFile.writeText(report)
            onProgress("Java 17 installation finished. Preparing selected Forge profile diagnostics...")
            return report + "\nSaved to files/runtime/java17-install-report.txt"
        } catch (error: Throwable) {
            if (staging.exists()) staging.deleteRecursively()
            throw error
        }
    }

    private fun findZalithSource(context: Context): Source {
        val checked = ArrayList<String>()
        for (packageName in sourcePackages) {
            try {
                val packageContext = context.createPackageContext(packageName, 0)
                val names = packageContext.assets.list(ASSET_ROOT)?.toSet() ?: emptySet()
                checked.add(packageName + " (assets found)")
                if (names.contains("universal.tar.xz") &&
                    names.contains("bin-arm64.tar.xz") &&
                    names.contains("version")
                ) {
                    return Source(packageName, packageContext)
                }
            } catch (_: Exception) {
                checked.add(packageName + " (not available)")
            }
        }
        throw IllegalStateException(
            "No installed ZalithLauncher2 package with Java 17 ARM64 assets was found. Checked: " +
                checked.joinToString(", ")
        )
    }

    private fun extractTarXz(
        input: java.io.InputStream,
        destination: File,
        budget: ExtractionBudget,
        label: String,
        onProgress: (String) -> Unit
    ) {
        val root = destination.canonicalFile
        val buffer = ByteArray(64 * 1024)
        TarArchiveInputStream(XZCompressorInputStream(BufferedInputStream(input))).use { tar ->
            while (true) {
                val entry = tar.nextTarEntry ?: break
                if (!tar.canReadEntryData(entry)) {
                    throw IllegalStateException("Unsupported data in runtime archive entry: " + entry.name)
                }
                budget.addEntry(entry.size)
                val target = safeResolve(root, entry.name)
                if (target == root) continue

                target.parentFile?.let {
                    if (!it.exists() && !it.mkdirs()) {
                        throw IllegalStateException("Cannot create runtime directory: " + it.absolutePath)
                    }
                }

                when {
                    entry.isSymbolicLink -> {
                        validateSymbolicLink(root, target, entry.linkName)
                        Files.deleteIfExists(target.toPath())
                        Os.symlink(entry.linkName, target.absolutePath)
                    }
                    entry.isLink -> {
                        val linkSource = safeResolve(root, entry.linkName)
                        if (!linkSource.exists()) {
                            throw IllegalStateException("Runtime archive hard link target is missing: " + entry.linkName)
                        }
                        Files.deleteIfExists(target.toPath())
                        Os.link(linkSource.absolutePath, target.absolutePath)
                    }
                    entry.isDirectory -> {
                        if (!target.exists() && !target.mkdirs()) {
                            throw IllegalStateException("Cannot create runtime directory: " + target.absolutePath)
                        }
                        applyMode(target, entry.mode)
                    }
                    entry.isFile -> {
                        Files.deleteIfExists(target.toPath())
                        FileOutputStream(target).use { output ->
                            while (true) {
                                val count = tar.read(buffer)
                                if (count < 0) break
                                if (count == 0) continue
                                output.write(buffer, 0, count)
                            }
                        }
                        applyMode(target, entry.mode)
                    }
                }
                if (budget.entries % 250 == 0) {
                    onProgress(label + ": " + budget.entries + " archive entries processed.")
                }
            }
        }
    }

    private fun safeResolve(root: File, name: String): File {
        val normalizedName = name.replace('\\', '/')
        if (normalizedName.startsWith("/") ||
            Regex("^[A-Za-z]:").containsMatchIn(normalizedName)
        ) {
            throw IllegalStateException("Unsafe absolute path in runtime archive: " + name)
        }
        val relative = Paths.get(normalizedName).normalize()
        if (relative.isAbsolute || relative.startsWith("..")) {
            throw IllegalStateException("Unsafe path in runtime archive: " + name)
        }
        val candidate = File(root, relative.toString()).canonicalFile
        val rootPath = root.canonicalPath
        if (candidate.path != rootPath && !candidate.path.startsWith(rootPath + File.separator)) {
            throw IllegalStateException("Runtime archive path escapes its installation directory: " + name)
        }
        return candidate
    }

    private fun validateSymbolicLink(root: File, link: File, linkName: String) {
        val linkTarget = Paths.get(linkName)
        if (linkTarget.isAbsolute) {
            throw IllegalStateException("Absolute symbolic link in runtime archive: " + linkName)
        }
        val resolved = link.parentFile.toPath().resolve(linkTarget).normalize()
        val rootPath: Path = root.canonicalFile.toPath()
        if (!resolved.startsWith(rootPath)) {
            throw IllegalStateException("Symbolic link escapes runtime directory: " + linkName)
        }
    }

    private fun applyMode(file: File, mode: Int) {
        file.setReadable(true, true)
        if ((mode and 0x92) != 0) file.setWritable(true, true)
        if ((mode and 0x49) != 0) file.setExecutable(true, true)
    }

    private fun enableBinExecutables(runtime: File) {
        val bin = File(runtime, "bin")
        if (!bin.isDirectory) {
            throw IllegalStateException("The Java 17 runtime has no bin/ directory.")
        }
        bin.walkTopDown().filter { it.isFile }.forEach { it.setExecutable(true, true) }
    }

    private fun testJava(runtime: File, context: Context): String {
        val java = File(runtime, "bin/java")
        val outputFile = File(context.cacheDir, "rustchunk-java17-version.txt")
        try {
            java.setExecutable(true, true)
            val builder = ProcessBuilder(java.absolutePath, "-version")
                .directory(runtime)
                .redirectErrorStream(true)
                .redirectOutput(outputFile)
            val environment = builder.environment()
            environment["JAVA_HOME"] = runtime.absolutePath
            val libraryPaths = listOf(
                context.applicationInfo.nativeLibraryDir,
                File(runtime, "lib").absolutePath,
                File(runtime, "lib/server").absolutePath,
                File(runtime, "lib/aarch64/server").absolutePath
            )
            val existing = environment["LD_LIBRARY_PATH"]
            if (!existing.isNullOrBlank()) libraryPaths.plus(existing)
            environment["LD_LIBRARY_PATH"] = (libraryPaths + listOfNotNull(existing)).distinct().joinToString(":")
            val process = builder.start()
            if (!process.waitFor(20, TimeUnit.SECONDS)) {
                process.destroyForcibly()
                process.waitFor(2, TimeUnit.SECONDS)
                return "RESULT: JAVA EXECUTION TIMEOUT (20 seconds)"
            }
            val output = if (outputFile.isFile) outputFile.readText(Charsets.UTF_8).trim() else ""
            val ok = process.exitValue() == 0 && output.contains("version", ignoreCase = true)
            return buildString {
                appendLine("Exit code: " + process.exitValue())
                if (output.isNotBlank()) appendLine(output.take(2500))
                append(if (ok) "RESULT: JAVA EXECUTION OK" else "RESULT: JAVA EXECUTION FAILED")
            }
        } catch (error: Throwable) {
            return "RESULT: JAVA EXECUTION ERROR\n" +
                error.javaClass.simpleName + ": " + error.message
        } finally {
            outputFile.delete()
        }
    }
}
