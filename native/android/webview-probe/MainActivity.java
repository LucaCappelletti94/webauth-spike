package com.connetto.webviewprobe;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;
import android.webkit.ValueCallback;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.LinearLayout;
import android.widget.TextView;

// A5: does an Android WebView expose WebAuthn at all. Loads the probe page over localhost
// and, once loaded, checks the WebAuthn globals. The result is both shown on screen and
// logged under the A5PROBE tag so it can be read over adb logcat.
public final class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        final TextView tv = new TextView(this);
        tv.setTextSize(16);
        tv.setPadding(24, 24, 24, 24);
        tv.setText("loading...");

        WebView web = new WebView(this);
        WebSettings settings = web.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        web.setWebViewClient(new WebViewClient() {
            @Override
            public void onPageFinished(WebView view, String url) {
                String js = "JSON.stringify({"
                    + "hasPublicKeyCredential: (typeof window.PublicKeyCredential !== 'undefined'),"
                    + "hasCredentials: (typeof navigator.credentials !== 'undefined'),"
                    + "hasCredentialsCreate: (!!(navigator.credentials && navigator.credentials.create)),"
                    + "hasCredentialsGet: (!!(navigator.credentials && navigator.credentials.get)),"
                    + "isSecureContext: window.isSecureContext,"
                    + "ua: navigator.userAgent"
                    + "})";
                view.evaluateJavascript(js, new ValueCallback<String>() {
                    @Override
                    public void onReceiveValue(String value) {
                        Log.i("A5PROBE", "webauthn-check=" + value);
                        tv.setText("A5 WebView WebAuthn check:\n\n" + value);
                    }
                });
            }
        });

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.addView(tv, new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT));
        root.addView(web, new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.MATCH_PARENT));
        setContentView(root);

        web.loadUrl("http://localhost:8000/browser/");
    }
}
