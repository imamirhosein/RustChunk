package dev.rustchunk.app

import android.content.Context
import android.net.Uri
import android.provider.DocumentsContract
import java.io.File
import java.io.FileOutputStream

/**
 * Imports only game/runtime data from a user-selected Minecraft game directory.
 * Authentication databases, launcher settings, logs and private account data are never copied.
 */
object ZalithImporter {
    data class Progress(
        val filesCopied: Int,
        val bytesCopied: Long,
        val currentPath: String
    )

    private data class Entry(
        val id: String,
        val name: String,
        val mimeType: String,
        val size: Long
    ) {
        val isDirectory: Boolean
            get() = mimeType == DocumentsContract.Document.MIME_TYPE_DIR
    }

    private val baseDirectories = setOf("assets", "libraries", "versions")
    private val importDirectories = setOf(
        "assets",
        "libraries",
        "versions",
        "mods",
        "config",
        "defaultconfigs",
        "resourcepacks",
        "shaderpacks",
        "kubejs",
        "scripts",
        "patchouli_books",
        "journeymap",
        "xaerominimap",
        "xaeroworldmap"
    )
    private val importRootFiles = setOf("options.txt", "servers.dat", "servers.dat_old")

    fun import(
        context: Context,
        selectedTree: Uri,
        destinationRoot: File,
        onProgress: (Progress) -> Unit
    ): String {
        val resolver = context.contentResolver
        val rootId = DocumentsContract.getTreeDocumentId(selectedTree)
        var sourceId = rootId
        var sourceEntries = listChildren(resolver, selectedTree, sourceId)

        // Some launchers expose a wrapper folder instead of the actual .minecraft directory.
        if (baseDirectoryCount(sourceEntries) < 2) {
            val candidate = sourceEntries.firstOrNull {
                it.isDirectory && it.name.lowercase() in setOf(".minecraft", "minecraft", "gamedir", "game")
            }
            if (candidate != null) {
                sourceId = candidate.id
                sourceEntries = listChildren(resolver, selectedTree, sourceId)
            }
        }

        val foundBaseDirectories = baseDirectoryCount(sourceEntries)
        if (foundBaseDirectories < 2) {
            throw IllegalArgumentException(
                "This folder does not look like a Minecraft game root. " +
                    "Choose the folder that directly contains at least two of: assets, libraries, versions."
            )
        }

        destinationRoot.mkdirs()
        val stats = MutableStats()
        onProgress(Progress(0, 0L, "Scanning selected Zalith game folder..."))

        for (entry in sourceEntries) {
            if (entry.isDirectory && entry.name in importDirectories) {
                val target = File(destinationRoot, safeName(entry.name))
                copyDirectory(
                    resolver,
                    selectedTree,
                    entry.id,
                    target,
                    entry.name,
                    stats,
                    onProgress
                )
            } else if (!entry.isDirectory && entry.name in importRootFiles) {
                copyFile(
                    resolver,
                    DocumentsContract.buildDocumentUriUsingTree(selectedTree, entry.id),
                    File(destinationRoot, safeName(entry.name)),
                    entry.name,
                    stats,
                    onProgress
                )
            }
        }

        val versionRoot = File(destinationRoot, "versions")
        val forgeVersions = versionRoot.listFiles()
            ?.filter { it.isDirectory && MinecraftProfileMatcher.isMinecraft1201Forge(it.name) }
            ?.map { it.name }
            ?.sorted()
            .orEmpty()

        return buildString {
            appendLine("Zalith import completed.")
            appendLine("Files copied: ${stats.files}")
            appendLine("Data copied: ${formatBytes(stats.bytes)}")
            appendLine("Destination: ${destinationRoot.absolutePath}")
            if (forgeVersions.isNotEmpty()) {
                appendLine("Detected Forge profile(s):")
                forgeVersions.forEach { appendLine("  - $it") }
            } else {
                appendLine("No 1.20.1 Forge version folder was detected under versions/.")
            }
            append("Imported game data only; launcher accounts and authentication data were not copied.")
        }
    }

    /**
     * Imports a user-selected Java runtime directory into a separate app-private location.
     * Select the runtime root itself (or a parent containing the runtime), never the whole launcher data folder.
     */
    fun importRuntime(
        context: Context,
        selectedTree: Uri,
        destinationParent: File,
        onProgress: (Progress) -> Unit
    ): String {
        val resolver = context.contentResolver
        val rootId = DocumentsContract.getTreeDocumentId(selectedTree)
        val destination = File(destinationParent, "imported-${System.currentTimeMillis()}")
        if (!destination.mkdirs()) {
            throw IllegalStateException("Cannot create runtime destination: ${destination.absolutePath}")
        }
        val stats = MutableStats()
        onProgress(Progress(0, 0L, "Scanning selected Java runtime folder..."))
        copyDirectory(resolver, selectedTree, rootId, destination, "runtime", stats, onProgress)

        fun markBinExecutables(directory: File) {
            val children = directory.listFiles() ?: return
            for (child in children) {
                if (child.isDirectory) markBinExecutables(child)
                else if (child.isFile && child.parentFile?.name == "bin") child.setExecutable(true, true)
            }
        }
        markBinExecutables(destination)

        return buildString {
            appendLine("Java runtime import completed.")
            appendLine("Files copied: ${stats.files}")
            appendLine("Data copied: ${formatBytes(stats.bytes)}")
            appendLine("Destination: ${destination.absolutePath}")
            append("Execute permission was enabled for files in bin/; run profile analysis to check whether this runtime layout is recognized.")
        }
    }

    private class MutableStats {
        var files: Int = 0
        var bytes: Long = 0L
    }

    private fun baseDirectoryCount(entries: List<Entry>): Int =
        entries.count { it.isDirectory && it.name in baseDirectories }

    private fun listChildren(
        resolver: android.content.ContentResolver,
        tree: Uri,
        parentDocumentId: String
    ): List<Entry> {
        val childrenUri = DocumentsContract.buildChildDocumentsUriUsingTree(tree, parentDocumentId)
        val projection = arrayOf(
            DocumentsContract.Document.COLUMN_DOCUMENT_ID,
            DocumentsContract.Document.COLUMN_DISPLAY_NAME,
            DocumentsContract.Document.COLUMN_MIME_TYPE,
            DocumentsContract.Document.COLUMN_SIZE
        )
        val result = ArrayList<Entry>()
        resolver.query(childrenUri, projection, null, null, null)?.use { cursor ->
            val idColumn = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_DOCUMENT_ID)
            val nameColumn = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_DISPLAY_NAME)
            val mimeColumn = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_MIME_TYPE)
            val sizeColumn = cursor.getColumnIndex(DocumentsContract.Document.COLUMN_SIZE)
            while (cursor.moveToNext()) {
                val id = cursor.getString(idColumn) ?: continue
                val name = cursor.getString(nameColumn) ?: continue
                val mime = cursor.getString(mimeColumn) ?: "application/octet-stream"
                val size = if (sizeColumn >= 0 && !cursor.isNull(sizeColumn)) cursor.getLong(sizeColumn) else -1L
                result.add(Entry(id, name, mime, size))
            }
        } ?: throw IllegalStateException("Android could not read the selected folder.")
        return result
    }

    private fun copyDirectory(
        resolver: android.content.ContentResolver,
        tree: Uri,
        sourceId: String,
        destination: File,
        relativePath: String,
        stats: MutableStats,
        onProgress: (Progress) -> Unit
    ) {
        if (!destination.exists() && !destination.mkdirs()) {
            throw IllegalStateException("Cannot create destination folder: ${destination.absolutePath}")
        }
        val entries = listChildren(resolver, tree, sourceId)
        for (entry in entries) {
            val name = safeName(entry.name)
            val childRelative = "$relativePath/$name"
            val target = File(destination, name)
            if (entry.isDirectory) {
                if (!target.exists() && !target.mkdirs()) {
                    throw IllegalStateException("Cannot create destination folder: ${target.absolutePath}")
                }
                copyDirectory(resolver, tree, entry.id, target, childRelative, stats, onProgress)
            } else {
                val sourceUri = DocumentsContract.buildDocumentUriUsingTree(tree, entry.id)
                copyFile(resolver, sourceUri, target, childRelative, stats, onProgress)
            }
        }
    }

    private fun copyFile(
        resolver: android.content.ContentResolver,
        sourceUri: Uri,
        target: File,
        relativePath: String,
        stats: MutableStats,
        onProgress: (Progress) -> Unit
    ) {
        val parent = target.parentFile
            ?: throw IllegalStateException("Invalid import destination: ${target.absolutePath}")
        if (!parent.exists() && !parent.mkdirs()) {
            throw IllegalStateException("Cannot create destination folder: ${parent.absolutePath}")
        }

        val temporary = File(parent, ".${target.name}.rustchunk-importing")
        try {
            val input = resolver.openInputStream(sourceUri)
                ?: throw IllegalStateException("Cannot open source file: $relativePath")
            input.use { source ->
                FileOutputStream(temporary).use { output ->
                    val buffer = ByteArray(64 * 1024)
                    while (true) {
                        val count = source.read(buffer)
                        if (count < 0) break
                        output.write(buffer, 0, count)
                        stats.bytes += count.toLong()
                    }
                    output.fd.sync()
                }
            }

            if (target.exists() && !target.delete()) {
                throw IllegalStateException("Cannot replace existing file: ${target.absolutePath}")
            }
            if (!temporary.renameTo(target)) {
                throw IllegalStateException("Cannot finalize imported file: ${target.absolutePath}")
            }

            stats.files++
            onProgress(Progress(stats.files, stats.bytes, relativePath))
        } catch (error: Throwable) {
            temporary.delete()
            throw error
        }
    }

    private fun safeName(name: String): String {
        if (name.isBlank() || name == "." || name == ".." ||
            name.contains('/') || name.contains('\\') || name.contains('\u0000')
        ) {
            throw IllegalArgumentException("The selected provider returned an unsafe file name.")
        }
        return name
    }

    private fun formatBytes(bytes: Long): String {
        val mib = bytes.toDouble() / (1024.0 * 1024.0)
        return String.format(java.util.Locale.US, "%.1f MiB", mib)
    }
}
