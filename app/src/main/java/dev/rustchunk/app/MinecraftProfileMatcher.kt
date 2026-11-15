package dev.rustchunk.app

/**
 * Recognizes the naming styles used by Minecraft launchers for the same Forge profile.
 * Examples: "1.20.1-forge-47.4.5" and "1.20.1 Forge 47.4.22".
 */
internal object MinecraftProfileMatcher {
    private val minecraft1201Pattern = Regex("""(?<!\d)1[._-]20[._-]1(?!\d)""")

    fun isMinecraft1201Forge(name: String): Boolean =
        minecraft1201Pattern.containsMatchIn(name) && name.contains("forge", ignoreCase = true)
}
