package dev.rustchunk.app

import android.app.Activity
import android.os.Bundle
import android.os.Handler
import android.os.Looper
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
    private lateinit var cancelButton: Button
    private var downloadInProgress = false

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
            text = "RustChunk\n\n" + nativeStatusText + "\n\n" + workspaceStatus
            textSize = 15f
            setPadding(20, 20, 20, 20)
            setTextIsSelectable(true)
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

    private fun resolveMinecraftMetadata() {
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
                downloadButton.isEnabled = hasInstallPlan() && !downloadInProgress
            }
        }
    }

    private fun startMinecraftDownload() {
        if (downloadInProgress) return
        if (!hasInstallPlan()) {
            statusView.text = "No install plan found. Resolve Minecraft metadata first."
            downloadButton.isEnabled = false
            return
        }

        downloadInProgress = true
        downloadButton.isEnabled = false
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
        init {
            System.loadLibrary("rustchunk")
        }
    }
}
