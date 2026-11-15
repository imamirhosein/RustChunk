package dev.rustchunk.app

import java.io.File

/**
 * Read-only inspection of Minecraft data already copied into RustChunk private storage.
 * This does not modify game files and does not attempt to launch Minecraft.
 */
object GameDataInspector {
    fun inspect(gameRoot: File): String {
        if (!gameRoot.isDirectory) {
            return "RustChunk game folder does not exist. Initialize the workspace first."
        }

        val baseNames = listOf("assets", "libraries", "versions")
        val presentBase = baseNames.filter { File(gameRoot, it).isDirectory }
        val versionRoot = File(gameRoot, "versions")
        val versionFolders = versionRoot.listFiles()
            ?.filter { it.isDirectory }
            ?.sortedBy { it.name }
            .orEmpty()
        val forgeProfiles = versionFolders.filter {
            it.name.startsWith("1.20.1-forge-", ignoreCase = true)
        }
        val otherVersionFolders = versionFolders.filterNot { it in forgeProfiles }
        val modsRoot = File(gameRoot, "mods")
        val modJarCount = countJarFiles(modsRoot)
        val forgeLibraries = File(gameRoot, "libraries/net/minecraftforge/forge")
        val workspaceProfile = File(gameRoot, "instances/forge-1.20.1/profile.properties")
        val importReport = File(gameRoot, "import-report.txt")

        return buildString {
            appendLine("RustChunk game-data inspection")
            appendLine("Game folder: " + gameRoot.absolutePath)
            appendLine("Base folders: " + presentBase.size + "/3 (" + presentBase.joinToString(", ") + ")")
            appendLine("Version folders: " + versionFolders.size)
            appendLine("Minecraft 1.20.1 Forge profiles: " + forgeProfiles.size)

            for (profile in forgeProfiles) {
                val json = File(profile, profile.name + ".json")
                val localJar = File(profile, profile.name + ".jar")
                appendLine(
                    "  - " + profile.name +
                        " | JSON: " + presentOrMissing(json.isFile) +
                        " | local JAR: " + presentOrMissing(localJar.isFile)
                )
            }

            if (otherVersionFolders.isNotEmpty()) {
                appendLine("Other installed versions/loaders:")
                for (version in otherVersionFolders.take(15)) {
                    appendLine("  - " + version.name)
                }
                if (otherVersionFolders.size > 15) {
                    appendLine("  ... and " + (otherVersionFolders.size - 15) + " more")
                }
            }

            appendLine("Forge libraries: " + presentOrMissing(forgeLibraries.isDirectory))
            appendLine("Mods folder: " + presentOrMissing(modsRoot.isDirectory))
            appendLine("Mod JAR files found: " + modJarCount)
            appendLine("Config folder: " + presentOrMissing(File(gameRoot, "config").isDirectory))
            appendLine("RustChunk Forge workspace profile: " + presentOrMissing(workspaceProfile.isFile))
            appendLine("Persistent import report: " + presentOrMissing(importReport.isFile))
            appendLine()

            when {
                presentBase.size < 2 -> append(
                    "The Minecraft base folders are incomplete or have not been imported."
                )
                forgeProfiles.isEmpty() -> append(
                    "Game data is present, but no Minecraft 1.20.1 Forge version folder was detected."
                )
                else -> append(
                    "The expected game-data folders are present. This is a file check only; Forge has not been launched."
                )
            }
        }
    }

    private fun countJarFiles(root: File): Long {
        if (!root.isDirectory) return 0L

        val pending = ArrayList<File>()
        pending.add(root)
        var count = 0L

        while (pending.isNotEmpty()) {
            val current = pending.removeAt(pending.lastIndex)
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

    private fun presentOrMissing(present: Boolean): String =
        if (present) "PRESENT" else "MISSING"
}
