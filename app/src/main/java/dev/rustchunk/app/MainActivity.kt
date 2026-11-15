package dev.rustchunk.app

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.AtomicFile
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import java.io.File
import java.util.concurrent.Executors

class MainActivity : Activity() {
    private external fun nativeStatus(): String
    private external fun nativePrepareWorkspace(appDataDirectory: String): String
    private external fun nativeResolveMinecraft(appDataDirectory: String): String
    private external fun nativeDownloadMinecraft(appDataDirectory: String, parallelism: Int): String
    private external fun nativeDownloadProgress(): String
    private external fun nativeCancelDownload(): String

    private val worker = Executors.newSingleThreadExecutor()
    private val mainHandler = Handler(Looper.getMainLooper())
    private lateinit var statusView: TextView
    private lateinit var resolveButton: Button
    private lateinit var downloadButton: Button
    private lateinit var importButton: Button
    private lateinit var inspectButton: Button
    private lateinit var cancelButton: Button
    private var downloadInProgress = false
    private var importInProgress = false

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
            text = "RustChunk\n\n" + nativeStatusText + "\n\n" + workspaceStatus + loadSavedImportReport()
            textSize = 15f
            setPadding(20, 20, 20, 20)
            setTextIsSelectable(true)
        }

        importButton = Button(this).apply {
            text = "Import game files from Zalith"
            isEnabled = !importInProgress
            setOnClickListener { chooseZalithFolder() }
        }

        inspectButton = Button(this).apply {
            text = "Inspect imported game files"
            setOnClickListener { inspectImportedGameData() }
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
            addView(inspectButton, matchWrapParams())
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
        if (requestCode != REQUEST_ZALITH_FOLDER || resultCode != RESULT_OK) return

        val selectedUri: Uri = data?.data ?: run {
            statusView.text = "No folder was selected."
            return
        }

        try {
            contentResolver.takePersistableUriPermission(
                selectedUri,
                Intent.FLAG_GRANT_READ_URI_PERMISSION
            )
        } catch (_: Throwable) {
            // The current read grant is enough for this immediate import operation.
        }

        startZalithImport(selectedUri)
    }

    private fun startZalithImport(sourceTree: Uri) {
        if (downloadInProgress || importInProgress) return
        importInProgress = true
        importButton.isEnabled = false
        resolveButton.isEnabled = false
        downloadButton.isEnabled = false
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
                importButton.isEnabled = true
                inspectButton.isEnabled = true
                resolveButton.isEnabled = isWorkspaceReady(workspaceStatus)
                downloadButton.isEnabled = hasInstallPlan()
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
            val savedReport = loadSavedImportReport()
            runOnUiThread {
                statusView.text = inspection + savedReport
                inspectButton.isEnabled = true
            }
        }
    }

    private fun formatBytes(bytes: Long): String =
        String.format(java.util.Locale.US, "%.1f MiB", bytes.toDouble() / (1024.0 * 1024.0))

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
        inspectButton.isEnabled = false
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
                inspectButton.isEnabled = true
                downloadButton.isEnabled = hasInstallPlan()
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
        worker.shutdown()
        super.onDestroy()
    }

    companion object {
        private const val REQUEST_ZALITH_FOLDER = 4102

        init {
            System.loadLibrary("rustchunk")
        }
    }
}
