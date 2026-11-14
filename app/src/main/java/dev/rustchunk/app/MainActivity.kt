package dev.rustchunk.app

import android.app.Activity
import android.os.Bundle
import android.view.Gravity
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import java.util.concurrent.Executors

class MainActivity : Activity() {
    private external fun nativeStatus(): String
    private external fun nativePrepareWorkspace(appDataDirectory: String): String
    private external fun nativeResolveMinecraft(appDataDirectory: String): String

    private val worker = Executors.newSingleThreadExecutor()
    private lateinit var statusView: TextView
    private lateinit var resolveButton: Button

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val nativeStatusText = try {
            nativeStatus()
        } catch (error: Throwable) {
            "Rust native core could not start: ${error.javaClass.simpleName}: ${error.message}"
        }

        val workspaceStatus = try {
            nativePrepareWorkspace(filesDir.absolutePath)
        } catch (error: Throwable) {
            "Workspace initialization failed: ${error.javaClass.simpleName}: ${error.message}"
        }

        statusView = TextView(this).apply {
            text = "RustChunk\n\n$nativeStatusText\n\n$workspaceStatus"
            textSize = 15f
            setPadding(20, 20, 20, 20)
            setTextIsSelectable(true)
        }

        resolveButton = Button(this).apply {
            text = "Resolve Minecraft 1.20.1 metadata"
            isEnabled = workspaceStatus.startsWith("Minecraft workspace ready")
            setOnClickListener { resolveMinecraftMetadata() }
        }

        val content = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(18, 18, 18, 18)
            addView(
                resolveButton,
                LinearLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
            )
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

    private fun resolveMinecraftMetadata() {
        resolveButton.isEnabled = false
        resolveButton.text = "Resolving..."
        statusView.text = "Connecting to Mojang metadata services...\nThis step only reads metadata and builds a download plan."

        worker.execute {
            val result = try {
                nativeResolveMinecraft(filesDir.absolutePath)
            } catch (error: Throwable) {
                "Metadata resolution failed: ${error.javaClass.simpleName}: ${error.message}"
            }

            runOnUiThread {
                statusView.text = result
                resolveButton.text = "Resolve Minecraft 1.20.1 metadata"
                resolveButton.isEnabled = true
            }
        }
    }

    override fun onDestroy() {
        worker.shutdown()
        super.onDestroy()
    }

    companion object {
        init {
            System.loadLibrary("rustchunk")
        }
    }
}
