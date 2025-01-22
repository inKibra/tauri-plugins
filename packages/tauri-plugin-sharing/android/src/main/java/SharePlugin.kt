package com.inkibra.tauri.plugin.sharing

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class SharePayload {
    var text: String = ""
    var url: String = ""
}

@TauriPlugin
class SharePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Share(activity)

    @Command
    fun share(invoke: Invoke) {
        try {
            val payload = invoke.parseArgs(SharePayload::class.java)
            implementation.share(payload.text, payload.url)
            
            val ret = JSObject()
            ret.put("success", true)
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject("Failed to share: ${e.message}")
        }
    }
}
