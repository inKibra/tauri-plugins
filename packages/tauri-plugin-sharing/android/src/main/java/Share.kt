package com.inkibra.tauri.plugin.sharing

import android.content.Context
import android.content.Intent
import android.util.Log

class Share(private val context: Context) {
    fun share(text: String, url: String) {
        try {
            val sendIntent = Intent().apply {
                action = Intent.ACTION_SEND
                // Combine text and URL in a meaningful way
                val shareText = when {
                    text.isNotEmpty() && url.isNotEmpty() -> "$text\n$url"
                    text.isNotEmpty() -> text
                    url.isNotEmpty() -> url
                    else -> throw IllegalArgumentException("Both text and url cannot be empty")
                }
                
                putExtra(Intent.EXTRA_TEXT, shareText)
                type = "text/plain"
            }
            
            val shareIntent = Intent.createChooser(sendIntent, null).apply {
                addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
            }
            
            context.startActivity(shareIntent)
        } catch (e: Exception) {
            Log.e("SharePlugin", "Error sharing content: ${e.message}")
            throw e
        }
    }
}
