package com.connetto.probe;

import android.content.Context;

// Thin bridge to the Rust cdylib. The native symbol is
// Java_com_connetto_probe_ProbeBridge_runProbe, matching this package and class.
public final class ProbeBridge {
    static {
        System.loadLibrary("webauth_probe_android");
    }

    private ProbeBridge() {}

    // Runs the A6 Keystore probe and returns the report JSON.
    public static native String runProbe(Context context);
}
