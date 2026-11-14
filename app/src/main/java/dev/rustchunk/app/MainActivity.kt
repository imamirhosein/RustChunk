package dev.rustchunk.app

import android.app.Activity
import android.os.Bundle
import android.view.Gravity
import android.widget.TextView

class MainActivity : Activity() {
    private external fun nativeStatus(): String
    private external fun nativePrepareWorkspace(appDataDirectory: String): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val nativeStatusText = try {
            nativeStatus()
        } catch (error: Throwable) {
            "Rust native core could not start: ${error.javaClass.simpleName}: ${error.message}"
        }

        val workspaceStatusText = try {
            nativePrepareWorkspace(filesDir.absolutePath)
        } catch (error: Throwable) {
            "Workspace initialization failed: ${error.javaClass.simpleName}: ${error.message}"
        }

        val view = TextView(this).apply {
            text = "RustChunk\n\n$nativeStatusText\n\n$workspaceStatusText\n\nStage 1: Minecraft workspace initialization"
            textSize = 16f
            gravity = Gravity.CENTER
            setPadding(28, 32, 28, 32)
            setTextIsSelectable(true)
        }
        setContentView(view)
    }

    companion object {
        init {
            System.loadLibrary("rustchunk")
        }
    }
}
