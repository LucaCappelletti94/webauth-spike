# Report format

One JSON object per matrix row, so the run can be checked against the questions mechanically. `results.template.json` is the whole matrix from section 7 pre-filled with metadata and empty answers. Copy it to `results.json` and fill it in as you run.

Each object looks like this:

```json
{
  "os": "Ubuntu 24.04", "browser": "Chrome 141", "provider": "Google Password Manager",
  "leg": "js",
  "results": {
    "Q4": { "pass": true, "value": "32" },
    "Q5": { "pass": true, "value": "3f2a...  (both runs identical)" }
  },
  "reflect_fallbacks": ["prf results read via Reflect, typed getter returned undefined"],
  "notes": "chooser offered three options, picked Google Password Manager"
}
```

`leg` is `js`, `rust`, or `native`. Values compared across runs go in as lowercase hexadecimal so equality can be checked rather than asserted. `reflect_fallbacks` applies only to the Rust leg and is empty everywhere else. A `pass` of `null` means the question was not answered on that configuration, which is a valid outcome to record rather than to drop.

The browser pages build these objects for you. Run the questions, fill the metadata fields, press the emit button, and paste the emitted object into your `results.json` array. The native binaries print one object each on stdout.

The single most important field is `provider`. On Chrome a chooser appears and it is easy to test something other than what you intended, so record which option actually answered. Prose notes are worth more than a clean table: a pass that felt wrong, an unexpected prompt, or a chooser offering options you did not anticipate are all findings.

## Checking

```
python3 report/check.py results.json
```

This validates shape, not findings: every object carries the required fields, `leg` is one of the three legs, `reflect_fallbacks` is empty except on the Rust leg, and each result is a `{pass, value}` pair. It prints how many questions each object answered and exits non-zero on any malformed object.

## What to watch for

Two comparisons dominate the whole study. If the emulator or simulator disagrees with its physical device on any answer, say so first and loudly, because it invalidates every emulated result for that platform. If the synced software passkey rows fail while hardware and platform-authenticator rows pass, that is the most consequential possible result, and it reopens a decision taken on the assumption that they pass.
