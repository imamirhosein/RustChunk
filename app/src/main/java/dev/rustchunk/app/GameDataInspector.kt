package dev.rustchunk.app

import java.io.File
import java.util.ArrayDeque
import java.util.Locale

/**
 * Read-only inspection of Minecraft data already copied into RustChunk private storage.
 * This does not modify game files and does not attempt to launch Minecraft.
 */
object GameDataInspector {
    private val ignoredSearchDirectories = setOf("assets", "libraries")

    fun inspect(gameRoot: File): String {
        if (!gameRoot.isDirectory) {
            return "RustChunk game folder does not exist. Initialize the workspace first."
        }

        val rootEntries = gameRoot.listFiles()
            ?.sortedBy { it.name.lowercase(Locale.ROOT) }
            .orEmpty()
        val baseNames = listOf("assets", "libraries", "versions")
        val presentBase = baseNames.filter { File(gameRoot, it).isDirectory }

        val versionRoot = File(gameRoot, "versions")
        val versionFolders = versionRoot.listFiles()
            ?.filter { it.isDirectory }
            ?.sortedBy { it.name.lowercase(Locale.ROOT) }
            .orEmpty()
        val forgeProfiles = versionFolders.filter {
            MinecraftProfileMatcher.isMinecraft1201Forge(it.name)
        }
        val otherVersionFolders = versionFolders.filterNot { it in forgeProfiles }

        val modsDirectories = findNamedDirectories(gameRoot, "mods", 5)
        val configDirectories = findNamedDirectories(gameRoot, "config", 5)
        val forgeLibraries = File(gameRoot, "libraries/net/minecraftforge/forge")
        val workspaceProfile = File(gameRoot, "instances/forge-1.20.1/profile.properties")
        val importReport = File(gameRoot, "import-report.txt")

        return buildString {
            appendLine("RustChunk game-data inspection")
            appendLine("Game folder: " + gameRoot.absolutePath)
            appendLine("Base folders: " + presentBase.size + "/3 (" + presentBase.joinToString(", ") + ")")
            appendLine("Top-level entries:")
            for (entry in rootEntries.take(50)) {
                appendLine("  - " + entry.name + if (entry.isDirectory) "/" else "")
            }
            if (rootEntries.size > 50) {
                appendLine("  ... and " + (rootEntries.size - 50) + " more")
            }

            appendLine("Version folders: " + versionFolders.size)
            appendLine("Minecraft 1.20.1 Forge profiles: " + forgeProfiles.size)
            for (profile in forgeProfiles) {
                val files = profile.listFiles().orEmpty()
                val jsonFiles = files.filter { it.isFile && it.extension.equals("json", true) }.map { it.name }
                val jarFiles = files.filter { it.isFile && it.extension.equals("jar", true) }.map { it.name }
                appendLine(
                    "  - " + profile.name +
                        " | JSON: " + (jsonFiles.take(5).joinToString(", ").ifEmpty { "MISSING" }) +
                        " | JAR: " + (jarFiles.take(5).joinToString(", ").ifEmpty { "MISSING" })
                )
            }

            if (otherVersionFolders.isNotEmpty()) {
                appendLine("Other installed versions/loaders:")
                for (version in otherVersionFolders.take(20)) {
                    val files = version.listFiles().orEmpty()
                    val jsonCount = files.count { it.isFile && it.extension.equals("json", true) }
                    val jarCount = files.count { it.isFile && it.extension.equals("jar", true) }
                    appendLine("  - " + version.name + " | JSON files: " + jsonCount + " | JAR files: " + jarCount)
                }
                if (otherVersionFolders.size > 20) {
                    appendLine("  ... and " + (otherVersionFolders.size - 20) + " more")
                }
            }

            appendLine(
                "Forge libraries: " + presentOrMissing(forgeLibraries.isDirectory) +
                    if (forgeLibraries.isDirectory) " (" + (forgeLibraries.listFiles()?.size ?: 0) + " entries)" else ""
            )

            val rootMods = File(gameRoot, "mods")
            appendLine("Root mods folder: " + presentOrMissing(rootMods.isDirectory))
            appendLine("Mods directories found within search depth 5: " + modsDirectories.size)
            for (directory in modsDirectories.take(10)) {
                appendLine("  - " + relativePath(gameRoot, directory) + " | JAR files: " + countJarFiles(directory))
            }
            if (modsDirectories.size > 10) {
                appendLine("  ... and " + (modsDirectories.size - 10) + " more")
            }

            val rootConfig = File(gameRoot, "config")
            appendLine("Root config folder: " + presentOrMissing(rootConfig.isDirectory))
            appendLine("Config directories found within search depth 5: " + configDirectories.size)
            for (directory in configDirectories.take(10)) {
                appendLine("  - " + relativePath(gameRoot, directory))
            }
            if (configDirectories.size > 10) {
                appendLine("  ... and " + (configDirectories.size - 10) + " more")
            }

            appendLine(
                "RustChunk workspace profile: " + presentOrMissing(workspaceProfile.isFile) +
                    " (generated by RustChunk; not proof of an imported Zalith profile)"
            )
            if (importReport.isFile) {
                appendLine("Persistent Zalith import report: PRESENT")
            } else {
                appendLine(
                    "Persistent Zalith import report: MISSING (expected when the data was imported " +
                        "before Stage 5 added report saving; this does not mean the copied files are missing)"
                )
            }
            appendLine()

            when {
                presentBase.size < 2 -> append(
                    "The Minecraft base folders are incomplete or have not been imported."
                )
                forgeProfiles.isEmpty() -> append(
                    "Game data is present, but no 1.20.1 Forge profile folder was detected."
                )
                modsDirectories.isEmpty() -> append(
                    "The 1.20.1 Forge profile folder is detected, but no mods folder was found in the scanned game-data tree."
                )
                else -> append(
                    "The expected game-data paths were found. This is a file-presence check only; Forge has not been launched."
                )
            }
        }
    }

    private fun findNamedDirectories(root: File, targetName: String, maxDepth: Int): List<File> {
        val found = ArrayList<File>()
        val pending = ArrayDeque<Pair<File, Int>>()
        val visited = HashSet<String>()
        pending.add(root to 0)

        while (pending.isNotEmpty() && visited.size < 2500) {
            val (current, depth) = pending.removeFirst()
            if (!visited.add(current.absolutePath)) continue

            if (current.name.equals(targetName, ignoreCase = true)) {
                found.add(current)
                continue
            }
            if (depth >= maxDepth) continue

            val children = current.listFiles()?.filter { it.isDirectory }.orEmpty()
            for (child in children) {
                if (child.name.lowercase(Locale.ROOT) in ignoredSearchDirectories) continue
                pending.add(child to depth + 1)
            }
        }

        return found.sortedBy { it.absolutePath.lowercase(Locale.ROOT) }
    }

    private fun countJarFiles(root: File): Long {
        if (!root.isDirectory) return 0L

        val pending = ArrayDeque<File>()
        val visited = HashSet<String>()
        pending.add(root)
        var count = 0L

        while (pending.isNotEmpty() && visited.size < 5000) {
            val current = pending.removeFirst()
            if (!visited.add(current.absolutePath)) continue
            val children = current.listFiles() ?: continue
            for (child in children) {
                if (child.isDirectory) {
                    pending.add(child)
                } else if (child.isFile && child.name.endsWith(".jar", ignoreCase = true)) {
                    count++
                }
            }
        }
        return count
    }

    private fun relativePath(root: File, file: File): String {
        val rootPath = root.absolutePath.trimEnd(File.separatorChar)
        val filePath = file.absolutePath
        return if (filePath.startsWith(rootPath + File.separator)) {
            filePath.substring(rootPath.length + 1)
        } else {
            filePath
        }
    }

    private fun presentOrMissing(present: Boolean): String =
        if (present) "PRESENT" else "MISSING"
}
