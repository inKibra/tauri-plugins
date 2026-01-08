package com.plugin.notifications

import android.Manifest
import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.google.firebase.FirebaseApp
import com.google.firebase.messaging.FirebaseMessaging

@TauriPlugin(
        permissions =
                [
                        Permission(
                                strings = [Manifest.permission.POST_NOTIFICATIONS],
                                alias = "postNotification"
                        )]
)
class NotificationsPlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun initFirebase(invoke: Invoke) {
        Log.i("notifyPlugin", "initializing firebase")
        FirebaseApp.initializeApp(this.activity)
        invoke.resolve()
    }

    @Command
    override fun checkPermissions(invoke: Invoke) {
        val ret = JSObject()
        ret.put("status", getPermissionState("postNotification"))
        invoke.resolve(ret)
    }

    @Command
    override fun requestPermissions(invoke: Invoke) {
        this.requestPermissionForAlias("postNotification", invoke, "notifyPermissionRequestDone")
    }

    @PermissionCallback
    fun notifyPermissionRequestDone(invoke: Invoke) {
        Log.i("notifyPlugin", "Permission request done")
        val ret = JSObject()
        ret.put("status", getPermissionState("postNotification"))
        invoke.resolve(ret)
    }

    @Command
    fun registerForRemoteNotifications(invoke: Invoke) {
        Log.i("notifyPlugin", "Registering for notifications")
        FirebaseMessaging.getInstance()
                .getToken()
                .addOnCompleteListener { task ->
                    val ret = JSObject()
                    if (!task.isSuccessful) {
                        Log.e("notifyPlugin", "Could not get firebase token.")
                        ret.put("success", false)
                        ret.put("error", "Failed to get firebase token")
                    } else {
                        // Get new Instance ID token
                        val token: String = task.result
                        ret.put("success", true)
                        ret.put("token", token)
                    }

                    invoke.resolve(ret)
                }
                .addOnCanceledListener { invoke.reject("Task cancelled") }
    }
}
