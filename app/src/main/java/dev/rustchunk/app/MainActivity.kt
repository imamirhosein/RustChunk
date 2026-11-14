package dev.rustchunk.app

import android.app.Activity
import android.os.Bundle
import android.view.Gravity
import android.widget.TextView

class MainActivity : Activity() {
    private external fun nativeStatus(): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val status = try {
            nativeStatus()
        } catch (error: Throwable) {
            "Rust native core could not start: ${error.javaClass.simpleName}: ${error.message}"
        }

        val view = TextView(this).apply {
            text = "RustChunk\n\n$status\n\nStage 0: Android ↔ Rust bridge test"
            textSize = 18f
            gravity = Gravity.CENTER
            setPadding(32, 32, 32, 32)
        }
        setContentView(view)
    }

    companion object {
        init {
            System.loadLibrary("rustchunk")
        }
    }
}
