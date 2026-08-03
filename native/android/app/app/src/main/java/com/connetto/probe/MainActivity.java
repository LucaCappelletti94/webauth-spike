package com.connetto.probe;

import android.app.Activity;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

// Minimal host: one button runs the native A6 probe, the result JSON is shown below it.
public final class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        int pad = (int) (16 * getResources().getDisplayMetrics().density);
        root.setPadding(pad, pad, pad, pad);

        Button run = new Button(this);
        run.setText("Run A6 probe");
        root.addView(run);

        final TextView out = new TextView(this);
        out.setTextIsSelectable(true);
        out.setGravity(Gravity.START);
        ScrollView scroll = new ScrollView(this);
        scroll.addView(out);
        root.addView(scroll);

        run.setOnClickListener(v -> {
            try {
                out.setText(ProbeBridge.runProbe(getApplicationContext()));
            } catch (Throwable t) {
                out.setText("probe threw: " + t);
            }
        });

        setContentView(root);
    }
}
