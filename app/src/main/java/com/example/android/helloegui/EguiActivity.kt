package com.example.android.helloegui

import android.app.NativeActivity
import android.util.Log

class EguiActivity : NativeActivity() {

    fun finishMe() {
        Log.e("EguiActivity", "finishMe")
        this.runOnUiThread { finish() }
    }
}