package dev.rustchunk.app

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.util.AtomicFile
import android.view.ViewGroup
import android.widget.AdapterView
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.LinearLayout
import android.widget.Spinner
import org.json.JSONArray
import android.widget.ScrollView
import android.widget.TextView
import java.io.File
import java.util.concurrent.Executors

class MainActivity : Activity() {
    private external fun nativeStatus(): String
    private external fun nativePrepareWorkspace(appDataDirectory: String): String
    private external fun nativeResolveMinecraft(appDataDirectory: String): String
    private external fun nativeListMinecraftProfiles(appDataDirectory: String): String
    private external fun nativePrepareForgeLaunchProfile(appDataDirectory: String, profileId: String): String
    private external fun nativeDownloadMinecraft(appDataDirectory: String, parallelism: Int): String
    private external fun nativeDownloadForgeLibraries(appDataDirectory: String, profileId: String, parallelism: Int): String
    private external fun nativeDownloadProgress(): String
    private external fun nativeCancelDownload(): String

    private val worker = Executors.newSingleThreadExecutor()
    private val mainHandler = Handler(Looper.getMainLooper())
    private lateinit var statusView: TextView
    private lateinit var resolveButton: Button
    private lateinit var downloadButton: Button
    private lateinit var importButton: Button
    private lateinit var inspectButton: Button
    private lateinit var forgeProfileButton: Button
    private lateinit var forgeDependencyButton: Button
    private lateinit var profileSpinner: Spinner
    private lateinit var refreshProfilesButton: Button
    private lateinit var runtimeImportButton: Button
    private lateinit var runtimeTestButton: Button
    private lateinit var runtimeInstallButton: Button
    private lateinit var cancelButton: Button
    private val profileOptions = ArrayList<ProfileOption>()
    private var selectedProfileId: String? = null
    private var downloadInProgress = false
    private var importInProgress = false
    private var runtimeTestStartedAt = 0L

    private data class ProfileOption(
        val id: String,
        val label: String,
        val supportedForForgeAnalysis: Boolean
    ) {
        override fun toString(): String = label
    }

    private val progressPoller = object : Runnable {
        override fun run() {
            if (!downloadInProgress) return
            statusView.text = try {
                nativeDownloadProgress()
            } catch (error: Throwable) {
                "Could not read download progress: " + error.javaClass.simpleName + ": " + error.message
            }
            if (downloadInProgress) {
                mainHandler.postDelayed(this, 500)
            }
        }
    }

    private val runtimeTestPoller = object : Runnable {
        override fun run() {
            if (runtimeTestStartedAt == 0L) return

            val reportFile = File(filesDir, "runtime/native-jli-test-report.txt")
            val report = try {
                if (reportFile.isFile) reportFile.readText(Charsets.UTF_8) else ""
            } catch (error: Throwable) {
                "Could not read JLI test report: " + error.message
            }

            if (report.startsWith("TEST STATUS: COMPLETED")) {
                statusView.text = report.removePrefix("TEST STATUS: COMPLETED\\n") +
                    "\\nSaved to files/runtime/native-jli-test-report.txt"
                runtimeTestStartedAt = 0L
                runtimeTestButton.isEnabled = true
                return
            }

            val elapsed = SystemClock.elapsedRealtime() - runtimeTestStartedAt
            if (elapsed >= 45_000L) {
                val consoleFile = File(filesDir, "runtime/native-jli-console.log")
                val console = try {
                    if (consoleFile.isFile) consoleFile.readText(Charsets.UTF_8).takeLast(7000)
                    else "No native JLI console log was produced."
                } catch (error: Throwable) {
                    "Could not read native JLI console log: " + error.message
                }
                statusView.text = buildString {
                    appendLine("The isolated Java test process did not return within 45 seconds.")
                    appendLine("RustChunk's main process is still running; the test was isolated from the UI.")
                    appendLine("The helper may have exited during JVM initialization.")
                    appendLine()
                    appendLine("Saved startup marker:")
                    appendLine(report.ifBlank { "No startup marker found." })
                    appendLine()
                    appendLine("Native JLI output:")
                    append(console)
                }
                runtimeTestStartedAt = 0L
                runtimeTestButton.isEnabled = true
                return
            }

            statusView.text = "Java 17 is being tested in a separate helper process.\\n" +
                "Elapsed: " + (elapsed / 1000L) + " seconds.\\n" +
                "The RustChunk interface remains isolated from JVM startup."
            mainHandler.postDelayed(this, 500)
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val nativeStatusText = try {
            nativeStatus()
        } catch (error: Throwable) {
            "Rust native core could not start: " + error.javaClass.simpleName + ": " + error.message
        }

        val workspaceStatus = try {
            nativePrepareWorkspace(filesDir.absolutePath)
        } catch (error: Throwable) {
            "Workspace initialization failed: " + error.javaClass.simpleName + ": " + error.message
        }

        statusView = TextView(this).apply {
            text = "RustChunk\n\n" + nativeStatusText + "\n\n" + workspaceStatus + loadSavedImportReport() + loadSavedInspectionReport() + loadSavedForgeProfileReport()
            textSize = 15f
            setPadding(20, 20, 20, 20)
            setTextIsSelectable(true)
        }

        importButton = Button(this).apply {
            text = "Import game files from Zalith"
            isEnabled = !importInProgress
            setOnClickListener { chooseZalithFolder() }
        }

        runtimeImportButton = Button(this).apply {
            text = "Import Android Java runtime folder"
            setOnClickListener { chooseRuntimeFolder() }
        }

        runtimeTestButton = Button(this).apply {
            text = "Test Java 17 using native JLI launcher"
            setOnClickListener { testJavaRuntime() }
        }

        runtimeInstallButton = Button(this).apply {
            text = "Install Java 17 from installed Zalith"
            setOnClickListener { installJava17FromZalith() }
        }

        profileSpinner = Spinner(this).apply {
            isEnabled = false
            onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
                override fun onItemSelected(parent: AdapterView<*>?, view: android.view.View?, position: Int, id: Long) {
                    selectedProfileId = profileOptions.getOrNull(position)?.id
                    updateForgeAnalyzeButton()
                }
                override fun onNothingSelected(parent: AdapterView<*>?) {
                    selectedProfileId = null
                    updateForgeAnalyzeButton()
                }
            }
        }

        refreshProfilesButton = Button(this).apply {
            text = "Refresh installed profiles"
            setOnClickListener { refreshMinecraftProfiles() }
        }

        inspectButton = Button(this).apply {
            text = "Inspect imported game files"
            setOnClickListener { inspectImportedGameData() }
        }

        forgeProfileButton = Button(this).apply {
            text = "Analyze selected Forge 1.20.1 profile"
            isEnabled = false
            setOnClickListener { prepareForgeLaunchProfile() }
        }

        forgeDependencyButton = Button(this).apply {
            text = "Download missing Forge libraries"
            isEnabled = false
            setOnClickListener { startForgeLibrariesDownload() }
        }

        resolveButton = Button(this).apply {
            text = "Resolve Minecraft 1.20.1 metadata"
            isEnabled = workspaceStatus.startsWith("Minecraft workspace ready")
            setOnClickListener { resolveMinecraftMetadata() }
        }

        downloadButton = Button(this).apply {
            text = "Download Minecraft base files"
            isEnabled = isWorkspaceReady(workspaceStatus) && hasInstallPlan()
            setOnClickListener { startMinecraftDownload() }
        }

        cancelButton = Button(this).apply {
            text = "Cancel download"
            isEnabled = false
            setOnClickListener {
                statusView.text = try {
                    nativeCancelDownload()
                } catch (error: Throwable) {
                    "Could not request cancellation: " + error.javaClass.simpleName + ": " + error.message
                }
            }
        }

        val content = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(18, 18, 18, 18)
            addView(importButton, matchWrapParams())
            addView(runtimeInstallButton, matchWrapParams())
            addView(runtimeImportButton, matchWrapParams())
            addView(runtimeTestButton, matchWrapParams())
            addView(refreshProfilesButton, matchWrapParams())
            addView(profileSpinner, matchWrapParams())
            addView(inspectButton, matchWrapParams())
            addView(forgeProfileButton, matchWrapParams())
            addView(forgeDependencyButton, matchWrapParams())
            addView(resolveButton, matchWrapParams())
            addView(downloadButton, matchWrapParams())
            addView(cancelButton, matchWrapParams())
            addView(
                statusView,
                LinearLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
            )
        }

        val scroll = ScrollView(this).apply {
            addView(
                content,
                ViewGroup.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
            )
        }
        setContentView(scroll)
        refreshMinecraftProfiles()
    }

    private fun matchWrapParams(): LinearLayout.LayoutParams =
        LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.WRAP_CONTENT
        )

    private fun isWorkspaceReady(status: String): Boolean =
        status.startsWith("Minecraft workspace ready")

    private fun hasInstallPlan(): Boolean =
        File(
            filesDir,
            "minecraft/instances/forge-1.20.1/install-plan.json"
        ).isFile

    private fun chooseZalithFolder() {
        if (downloadInProgress || importInProgress) return
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
        }
        try {
            startActivityForResult(intent, REQUEST_ZALITH_FOLDER)
        } catch (error: Throwable) {
            statusView.text = "Could not open Android folder picker: ${error.message}"
        }
    }

    @Deprecated("Legacy activity result is used to keep the app independent of AndroidX Activity.")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (resultCode != RESULT_OK) return
        val selectedUri: Uri = data?.data ?: run {
            statusView.text = "No folder was selected."
            return
        }
        try {
            contentResolver.takePersistableUriPermission(
                selectedUri, Intent.FLAG_GRANT_READ_URI_PERMISSION
            )
        } catch (_: Throwable) {
            // The current read grant is enough for this immediate import operation.
        }
        when (requestCode) {
            REQUEST_ZALITH_FOLDER -> startZalithImport(selectedUri)
            REQUEST_RUNTIME_FOLDER -> startRuntimeImport(selectedUri)
        }
    }

    private fun chooseRuntimeFolder() {
        if (downloadInProgress || importInProgress) return
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
        }
        try {
            startActivityForResult(intent, REQUEST_RUNTIME_FOLDER)
        } catch (error: Throwable) {
            statusView.text = "Could not open Android folder picker: ${error.message}"
        }
    }


    private fun installJava17FromZalith() {
        if (downloadInProgress || importInProgress) return
        importInProgress = true
        setImportControlsEnabled(false)
        val selectedProfileAtStart = selectedProfileId
        statusView.text = "Searching installed ZalithLauncher2 packages for their bundled Java 17 ARM64 runtime..."
        worker.execute {
            var result = ""
            var succeeded = false
            try {
                result = JavaRuntimeInstaller.installFromInstalledZalith(
                    this,
                    File(filesDir, "games/runtimes")
                ) { progress ->
                    runOnUiThread {
                        if (importInProgress) statusView.text = progress
                    }
                }
                succeeded = true
            } catch (error: Throwable) {
                result = "Java 17 installation failed: " +
                    error.javaClass.simpleName + ": " + error.message
            }

            val analysis = if (succeeded && !selectedProfileAtStart.isNullOrBlank()) {
                try {
                    nativePrepareForgeLaunchProfile(filesDir.absolutePath, selectedProfileAtStart)
                } catch (error: Throwable) {
                    "Java 17 was installed, but profile re-analysis failed: " + error.message
                }
            } else if (succeeded) {
                "Java 17 is installed. Select your Forge 1.20.1 profile and analyze it to check runtime detection."
            } else {
                "The existing game files and profiles were not changed."
            }

            runOnUiThread {
                importInProgress = false
                statusView.text = result + "\n\n" + analysis
                setImportControlsEnabled(true)
                updateForgeAnalyzeButton()
            }
        }
    }

    private fun startRuntimeImport(sourceTree: Uri) {
        if (downloadInProgress || importInProgress) return
        importInProgress = true
        setImportControlsEnabled(false)
        val selectedProfileAtStart = selectedProfileId
        statusView.text = "Importing the selected Java runtime directory...\nChoose the runtime folder itself, not the entire launcher folder."
        worker.execute {
            val result = try {
                ZalithImporter.importRuntime(this, sourceTree, File(filesDir, "runtime")) { progress ->
                    runOnUiThread {
                        statusView.text = buildString {
                            appendLine("Importing Java runtime...")
                            appendLine("Files copied: ${progress.filesCopied}")
                            appendLine("Data copied: ${formatBytes(progress.bytesCopied)}")
                            append("Current: ${progress.currentPath}")
                        }
                    }
                }
            } catch (error: Throwable) {
                "Java runtime import failed: ${error.javaClass.simpleName}: ${error.message}"
            }
            val analysis = if (result.startsWith("Java runtime import completed.") &&
                !selectedProfileAtStart.isNullOrBlank()
            ) {
                try {
                    nativePrepareForgeLaunchProfile(filesDir.absolutePath, selectedProfileAtStart)
                } catch (error: Throwable) {
                    "Runtime imported, but profile re-analysis failed: ${error.message}"
                }
            } else {
                "Select a supported Forge 1.20.1 profile and analyze it to check whether the runtime was detected."
            }
            runOnUiThread {
                importInProgress = false
                statusView.text = result + "\n\n" + analysis
                setImportControlsEnabled(true)
                updateForgeAnalyzeButton()
            }
        }
    }

    private fun setImportControlsEnabled(enabled: Boolean) {
        importButton.isEnabled = enabled
        runtimeInstallButton.isEnabled = enabled
        runtimeImportButton.isEnabled = enabled
        runtimeTestButton.isEnabled = enabled
        refreshProfilesButton.isEnabled = enabled
        inspectButton.isEnabled = enabled
        if (::forgeDependencyButton.isInitialized) {
            forgeDependencyButton.isEnabled = enabled &&
                profileOptions.getOrNull(profileSpinner.selectedItemPosition)?.supportedForForgeAnalysis == true
        }
        profileSpinner.isEnabled = enabled && profileOptions.isNotEmpty()
        resolveButton.isEnabled = enabled
        downloadButton.isEnabled = enabled && hasInstallPlan()
    }

    private fun refreshMinecraftProfiles() {
        if (importInProgress || downloadInProgress) return
        refreshProfilesButton.isEnabled = false
        statusView.text = "Scanning installed version profiles..."
        worker.execute {
            val result = try {
                val raw = nativeListMinecraftProfiles(filesDir.absolutePath)
                val json = JSONArray(raw)
                val parsed = ArrayList<ProfileOption>()
                for (index in 0 until json.length()) {
                    val item = json.optJSONObject(index) ?: continue
                    val id = item.optString("id")
                    if (id.isBlank() || id == "null") continue
                    val displayName = item.optString("display_name", id)
                    val loader = item.optString("loader", "Unknown")
                    val mcVersion = item.optString("minecraft_version", "unknown")
                    // Be tolerant of older or incomplete profile metadata. The profile ID
                    // itself is authoritative for this supported 1.20.1 Forge naming style.
                    val supportedByMetadata =
                        item.optBoolean("supported_for_forge_analysis", false)
                    val supportedByProfileId =
                        MinecraftProfileMatcher.isMinecraft1201Forge(id)
                    val supportedByFields =
                        loader.equals("Forge", ignoreCase = true) && mcVersion == "1.20.1"
                    parsed.add(ProfileOption(
                        id,
                        "$displayName  |  $loader  |  Minecraft $mcVersion",
                        supportedByMetadata || supportedByProfileId || supportedByFields
                    ))
                }
                Pair(parsed, null as String?)
            } catch (error: Throwable) {
                Pair(ArrayList<ProfileOption>(), "Profile scan failed: ${error.javaClass.simpleName}: ${error.message}")
            }
            runOnUiThread {
                profileOptions.clear()
                profileOptions.addAll(result.first)
                val adapter = ArrayAdapter(this, android.R.layout.simple_spinner_item, profileOptions)
                adapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item)
                profileSpinner.adapter = adapter
                profileSpinner.isEnabled = profileOptions.isNotEmpty() && !importInProgress

                // Android may defer Spinner's onItemSelected callback until after layout.
                // Set the selected profile ID explicitly so the action button is not left
                // disabled while waiting for that callback.
                val preferredIndex = profileOptions.indexOfFirst {
                    it.supportedForForgeAnalysis
                }
                val initialIndex = when {
                    preferredIndex >= 0 -> preferredIndex
                    profileOptions.isNotEmpty() -> 0
                    else -> -1
                }
                if (initialIndex >= 0) {
                    profileSpinner.setSelection(initialIndex, false)
                    selectedProfileId = profileOptions[initialIndex].id
                } else {
                    selectedProfileId = null
                }

                refreshProfilesButton.isEnabled = !importInProgress && !downloadInProgress
                if (profileOptions.isEmpty()) {
                    selectedProfileId = null
                    statusView.text = result.second ?: "No installed version profiles were found under minecraft/versions."
                } else if (result.second != null) {
                    statusView.text = result.second!!
                } else {
                    statusView.text = "Found ${profileOptions.size} profile(s). Select a Forge 1.20.1 profile to analyze."
                }
                updateForgeAnalyzeButton()
            }
        }
    }

    private fun updateForgeAnalyzeButton() {
        if (!::forgeProfileButton.isInitialized) return

        // Prefer the Spinner's actual selection; fall back to the explicitly stored ID
        // during the short interval before Android dispatches onItemSelected.
        val spinnerSelection = if (::profileSpinner.isInitialized) {
            profileOptions.getOrNull(profileSpinner.selectedItemPosition)
        } else {
            null
        }
        val selected = spinnerSelection
            ?: profileOptions.firstOrNull { it.id == selectedProfileId }

        selectedProfileId = selected?.id
        val canUseForgeProfile = !importInProgress && !downloadInProgress &&
            selected?.supportedForForgeAnalysis == true
        forgeProfileButton.isEnabled = canUseForgeProfile
        if (::forgeDependencyButton.isInitialized) {
            forgeDependencyButton.isEnabled = canUseForgeProfile
        }
    }

    private fun startZalithImport(sourceTree: Uri) {
        if (downloadInProgress || importInProgress) return
        importInProgress = true
        setImportControlsEnabled(false)
        statusView.text = "Importing game files from the selected Zalith folder...\nDo not close RustChunk during the copy."

        worker.execute {
            var result = try {
                ZalithImporter.import(
                    this,
                    sourceTree,
                    File(filesDir, "minecraft")
                ) { progress ->
                    runOnUiThread {
                        statusView.text = buildString {
                            appendLine("Importing Zalith game data...")
                            appendLine("Files copied: ${progress.filesCopied}")
                            appendLine("Data copied: ${formatBytes(progress.bytesCopied)}")
                            append("Current: ${progress.currentPath}")
                        }
                    }
                }
            } catch (error: Throwable) {
                "Zalith import failed: ${error.javaClass.simpleName}: ${error.message}"
            }

            if (result.startsWith("Zalith import completed.")) {
                try {
                    saveImportReport(result)
                    result += "\nPersistent report saved to files/minecraft/import-report.txt"
                } catch (error: Throwable) {
                    result += "\nWARNING: Import completed, but its report could not be saved: " + error.message
                }
            }

            val workspaceStatus = try {
                nativePrepareWorkspace(filesDir.absolutePath)
            } catch (error: Throwable) {
                "Workspace check failed: ${error.message}"
            }

            runOnUiThread {
                importInProgress = false
                statusView.text = result + "\n\n" + workspaceStatus
                setImportControlsEnabled(true)
                updateForgeAnalyzeButton()
                refreshMinecraftProfiles()
            }
        }
    }

    private fun saveImportReport(report: String) {
        val reportFile = File(filesDir, "minecraft/import-report.txt")
        val atomicFile = AtomicFile(reportFile)
        val output = atomicFile.startWrite()
        try {
            output.write(report.toByteArray(Charsets.UTF_8))
            output.fd.sync()
            atomicFile.finishWrite(output)
        } catch (error: Throwable) {
            atomicFile.failWrite(output)
            throw error
        }
    }

    private fun loadSavedImportReport(): String {
        val reportFile = File(filesDir, "minecraft/import-report.txt")
        if (!reportFile.isFile) {
            return "\n\nNo saved Zalith import report yet."
        }
        return try {
            "\n\nLast completed Zalith import:\n" +
                reportFile.readText(Charsets.UTF_8).take(10000)
        } catch (error: Throwable) {
            "\n\nA saved import report exists but could not be read: " + error.message
        }
    }

    private fun inspectImportedGameData() {
        if (downloadInProgress || importInProgress) return
        inspectButton.isEnabled = false
        statusView.text = "Inspecting the imported files in RustChunk private storage..."

        worker.execute {
            val inspection = try {
                GameDataInspector.inspect(File(filesDir, "minecraft"))
            } catch (error: Throwable) {
                "Game-data inspection failed: " + error.javaClass.simpleName + ": " + error.message
            }

            val reportStatus = try {
                saveInspectionReport(inspection)
                "\n\nInspection report saved to files/minecraft/inspection-report.txt"
            } catch (error: Throwable) {
                "\n\nWARNING: Inspection completed, but the report could not be saved: " + error.message
            }

            val savedReport = loadSavedImportReport()
            runOnUiThread {
                statusView.text = inspection + reportStatus + savedReport
                inspectButton.isEnabled = true
            }
        }
    }

    private fun saveInspectionReport(report: String) {
        val reportFile = File(filesDir, "minecraft/inspection-report.txt")
        val atomicFile = AtomicFile(reportFile)
        val output = atomicFile.startWrite()
        try {
            output.write(report.toByteArray(Charsets.UTF_8))
            output.fd.sync()
            atomicFile.finishWrite(output)
        } catch (error: Throwable) {
            atomicFile.failWrite(output)
            throw error
        }
    }

    private fun loadSavedForgeProfileReport(): String {
        val versionsRoot = File(filesDir, "minecraft/versions")
        val reports = versionsRoot.listFiles()
            ?.filter { it.isDirectory }
            ?.mapNotNull { directory ->
                val reportFile = File(directory, "rustchunk-readiness-report.txt")
                if (reportFile.isFile) directory.name to reportFile else null
            }
            ?.sortedBy { it.first.lowercase(java.util.Locale.ROOT) }
            .orEmpty()
        if (reports.isEmpty()) return "\n\nNo profile-specific Forge analysis report has been prepared yet."
        return buildString {
            for ((profileName, reportFile) in reports) {
                append("\n\nSaved Forge analysis for ")
                append(profileName)
                append(":\n")
                try { append(reportFile.readText(Charsets.UTF_8).take(8000)) }
                catch (error: Throwable) { append("Report exists but could not be read: ${error.message}") }
            }
        }
    }

    private fun loadSavedInspectionReport(): String {
        val reportFile = File(filesDir, "minecraft/inspection-report.txt")
        if (!reportFile.isFile) {
            return "\n\nNo saved inspection report yet."
        }
        return try {
            "\n\nLast saved game-data inspection:\n" +
                reportFile.readText(Charsets.UTF_8).take(10000)
        } catch (error: Throwable) {
            "\n\nA saved inspection report exists but could not be read: " + error.message
        }
    }

    private fun formatBytes(bytes: Long): String =
        String.format(java.util.Locale.US, "%.1f MiB", bytes.toDouble() / (1024.0 * 1024.0))

    private fun testJavaRuntime() {
        if (importInProgress || downloadInProgress || runtimeTestStartedAt != 0L) return
        runtimeTestButton.isEnabled = false

        val reportFile = File(filesDir, "runtime/native-jli-test-report.txt")
        try {
            if (!reportFile.parentFile!!.exists() && !reportFile.parentFile!!.mkdirs()) {
                throw IllegalStateException("Cannot create the runtime diagnostics directory.")
            }
            val atomicFile = AtomicFile(reportFile)
            val output = atomicFile.startWrite()
            try {
                output.write(
                    ("TEST STATUS: RUNNING\\n" +
                        "The test is executing in Android process dev.rustchunk.app:jvm.\\n")
                        .toByteArray(Charsets.UTF_8)
                )
                output.fd.sync()
                atomicFile.finishWrite(output)
            } catch (error: Throwable) {
                atomicFile.failWrite(output)
                throw error
            }

            val consoleFile = File(filesDir, "runtime/native-jli-console.log")
            if (consoleFile.exists()) consoleFile.delete()

            runtimeTestStartedAt = SystemClock.elapsedRealtime()
            statusView.text = "Starting Java 17 in a dedicated helper process...\\n" +
                "A JVM startup failure must not close the RustChunk interface."
            mainHandler.removeCallbacks(runtimeTestPoller)
            startService(
                Intent(this, JavaRuntimeTestService::class.java)
                    .setAction(JavaRuntimeTestService.ACTION_RUN_TEST)
            )
            mainHandler.postDelayed(runtimeTestPoller, 400)
        } catch (error: Throwable) {
            runtimeTestStartedAt = 0L
            runtimeTestButton.isEnabled = true
            statusView.text = "Could not start the isolated Java test: " +
                error.javaClass.simpleName + ": " + error.message
        }
    }


    private fun prepareForgeLaunchProfile() {
        if (importInProgress || downloadInProgress) return
        val profileId = selectedProfileId
        if (profileId.isNullOrBlank()) {
            statusView.text = "Select a Forge 1.20.1 profile first."
            return
        }
        forgeProfileButton.isEnabled = false
        statusView.text = "Analyzing selected profile: $profileId\nThis step does not launch Minecraft or modify imported game files."
        worker.execute {
            val result = try {
                nativePrepareForgeLaunchProfile(filesDir.absolutePath, profileId)
            } catch (error: Throwable) {
                "Forge profile preparation failed: " + error.javaClass.simpleName + ": " + error.message
            }
            runOnUiThread {
                statusView.text = result
                updateForgeAnalyzeButton()
            }
        }
    }

    private fun resolveMinecraftMetadata() {
        if (importInProgress || downloadInProgress) return
        resolveButton.isEnabled = false
        resolveButton.text = "Resolving..."
        statusView.text = "Connecting to Mojang metadata services...\nThis step only reads metadata and builds a download plan."

        worker.execute {
            val result = try {
                nativeResolveMinecraft(filesDir.absolutePath)
            } catch (error: Throwable) {
                "Metadata resolution failed: " + error.javaClass.simpleName + ": " + error.message
            }

            runOnUiThread {
                statusView.text = result
                resolveButton.text = "Resolve Minecraft 1.20.1 metadata"
                resolveButton.isEnabled = true
                downloadButton.isEnabled = hasInstallPlan() && !downloadInProgress && !importInProgress
            }
        }
    }

    private fun startMinecraftDownload() {
        if (downloadInProgress || importInProgress) return
        if (!hasInstallPlan()) {
            statusView.text = "No install plan found. Resolve Minecraft metadata first."
            downloadButton.isEnabled = false
            return
        }

        downloadInProgress = true
        downloadButton.isEnabled = false
        importButton.isEnabled = false
        runtimeImportButton.isEnabled = false
        runtimeTestButton.isEnabled = false
        refreshProfilesButton.isEnabled = false
        profileSpinner.isEnabled = false
        inspectButton.isEnabled = false
        forgeProfileButton.isEnabled = false
        forgeDependencyButton.isEnabled = false
        cancelButton.isEnabled = true
        statusView.text = "Preparing download...\nThe app will verify every file with SHA-1.\nPartial files are kept for resume."
        mainHandler.removeCallbacks(progressPoller)
        mainHandler.post(progressPoller)

        worker.execute {
            val result = try {
                nativeDownloadMinecraft(filesDir.absolutePath, 4)
            } catch (error: Throwable) {
                "Download failed: " + error.javaClass.simpleName + ": " + error.message
            }

            runOnUiThread {
                downloadInProgress = false
                mainHandler.removeCallbacks(progressPoller)
                statusView.text = result
                cancelButton.isEnabled = false
                importButton.isEnabled = true
                runtimeImportButton.isEnabled = true
                runtimeTestButton.isEnabled = true
                refreshProfilesButton.isEnabled = true
                profileSpinner.isEnabled = profileOptions.isNotEmpty()
                inspectButton.isEnabled = true
                updateForgeAnalyzeButton()
                downloadButton.isEnabled = hasInstallPlan()
            }
        }
    }

    private fun startForgeLibrariesDownload() {
        if (downloadInProgress || importInProgress) return
        val profileId = selectedProfileId
        if (profileId.isNullOrBlank()) {
            statusView.text = "Select a supported Forge 1.20.1 profile first."
            return
        }

        downloadInProgress = true
        downloadButton.isEnabled = false
        forgeProfileButton.isEnabled = false
        forgeDependencyButton.isEnabled = false
        importButton.isEnabled = false
        runtimeInstallButton.isEnabled = false
        runtimeImportButton.isEnabled = false
        runtimeTestButton.isEnabled = false
        refreshProfilesButton.isEnabled = false
        profileSpinner.isEnabled = false
        inspectButton.isEnabled = false
        cancelButton.isEnabled = true
        statusView.text = "Preparing the selected Forge profile's Java-library plan...\nExisting files will be reused only after size and SHA-1 verification."
        mainHandler.removeCallbacks(progressPoller)
        mainHandler.post(progressPoller)

        worker.execute {
            val result = try {
                nativeDownloadForgeLibraries(filesDir.absolutePath, profileId, 4)
            } catch (error: Throwable) {
                "Forge dependency download failed: " + error.javaClass.simpleName + ": " + error.message
            }
            val analysis = try {
                nativePrepareForgeLaunchProfile(filesDir.absolutePath, profileId)
            } catch (error: Throwable) {
                "Post-download profile analysis failed: " + error.javaClass.simpleName + ": " + error.message
            }

            runOnUiThread {
                downloadInProgress = false
                mainHandler.removeCallbacks(progressPoller)
                statusView.text = result + "\n\nPost-download profile check:\n" + analysis
                cancelButton.isEnabled = false
                importButton.isEnabled = true
                runtimeInstallButton.isEnabled = true
                runtimeImportButton.isEnabled = true
                runtimeTestButton.isEnabled = true
                refreshProfilesButton.isEnabled = true
                profileSpinner.isEnabled = profileOptions.isNotEmpty()
                inspectButton.isEnabled = true
                resolveButton.isEnabled = true
                downloadButton.isEnabled = hasInstallPlan()
                updateForgeAnalyzeButton()
            }
        }
    }

    override fun onDestroy() {
        if (downloadInProgress) {
            try {
                nativeCancelDownload()
            } catch (_: Throwable) {
                // Activity is closing; there is no UI left to report a cancellation error.
            }
        }
        mainHandler.removeCallbacks(progressPoller)
        mainHandler.removeCallbacks(runtimeTestPoller)
        worker.shutdown()
        super.onDestroy()
    }

    companion object {
        private const val REQUEST_ZALITH_FOLDER = 4102
        private const val REQUEST_RUNTIME_FOLDER = 4103

        init {
            System.loadLibrary("rustchunk")
        }
    }
}
