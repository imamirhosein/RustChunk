package dev.rustchunk.app

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.util.AtomicFile
import java.io.File
import java.util.concurrent.Executors

/**
 * Runs embedded HotSpot in its own Android process.
 *
 * OpenJDK's JLI_Launch can call process-wide exit(1) when VM initialization
 * fails. Keeping it in :jvm prevents a failed JVM startup from terminating
 * RustChunk's UI process or discarding the diagnostic screen.
 */
class JavaRuntimeTestService : Service() {
    private val executor = Executors.newSingleThreadExecutor()

    private external fun nativeTestJavaRuntimeNative(
        appDataDirectory: String,
        nativeLibraryDirectory: String
    ): String

    override fun onCreate() {
        super.onCreate()
        System.loadLibrary("rustchunk")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (intent?.action != ACTION_RUN_TEST) {
            stopSelf(startId)
            return START_NOT_STICKY
        }

        executor.execute {
            val reportFile = File(filesDir, "runtime/native-jli-test-report.txt")
            try {
                writeAtomic(
                    reportFile,
                    "TEST STATUS: RUNNING\n" +
                        "Helper process: " + ApplicationProcessName.value(this) + "\n" +
                        "Helper PID: " + android.os.Process.myPid() + "\n"
                )
                val result = nativeTestJavaRuntimeNative(
                    filesDir.absolutePath,
                    applicationInfo.nativeLibraryDir
                )
                writeAtomic(reportFile, "TEST STATUS: COMPLETED\n" + result)
            } catch (error: Throwable) {
                val result = "TEST STATUS: COMPLETED\n" +
                    "RESULT: NATIVE JLI STARTUP ERROR\n" +
                    error.javaClass.simpleName + ": " + error.message
                runCatching { writeAtomic(reportFile, result) }
            } finally {
                stopSelf(startId)
            }
        }
        return START_NOT_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        executor.shutdownNow()
        super.onDestroy()
    }

    private fun writeAtomic(file: File, value: String) {
        file.parentFile?.let {
            if (!it.exists() && !it.mkdirs()) {
                throw IllegalStateException("Cannot create diagnostics directory: " + it.absolutePath)
            }
        }
        val atomic = AtomicFile(file)
        val output = atomic.startWrite()
        try {
            output.write(value.toByteArray(Charsets.UTF_8))
            output.fd.sync()
            atomic.finishWrite(output)
        } catch (error: Throwable) {
            atomic.failWrite(output)
            throw error
        }
    }

    companion object {
        const val ACTION_RUN_TEST = "dev.rustchunk.app.action.RUN_NATIVE_JLI_TEST"
    }
}

/** Resolves this service's actual process name without depending on AndroidX. */
private object ApplicationProcessName {
    fun value(service: Service): String {
        return try {
            val manager = service.getSystemService(android.app.ActivityManager::class.java)
            manager.runningAppProcesses
                ?.firstOrNull { it.pid == android.os.Process.myPid() }
                ?.processName ?: "dev.rustchunk.app:jvm"
        } catch (_: Throwable) {
            "dev.rustchunk.app:jvm"
        }
    }
}
