#!/usr/bin/env python3
"""Validate a probe results file against the report format.

Usage: python3 report/check.py <results.json>

The file is a JSON array of report objects, one per matrix row and leg. This checks
shape, not findings: every object carries the required fields, leg is one of the three
legs, reflect_fallbacks is empty except on the rust leg, and each result is a
{pass, value} pair whose pass is true, false, or null (null meaning not-answered).
It prints a per-object summary and exits non-zero if any object is malformed.
"""
import json
import sys

LEGS = {"js", "rust", "native"}
REQUIRED = {"os", "browser", "provider", "leg", "results", "reflect_fallbacks", "notes"}


def check_object(i, obj):
    errors = []
    if not isinstance(obj, dict):
        return [f"row {i}: not an object"]
    missing = REQUIRED - obj.keys()
    if missing:
        errors.append(f"row {i}: missing fields {sorted(missing)}")
        return errors
    if obj["leg"] not in LEGS:
        errors.append(f"row {i}: leg {obj['leg']!r} not one of {sorted(LEGS)}")
    if not isinstance(obj["reflect_fallbacks"], list):
        errors.append(f"row {i}: reflect_fallbacks is not a list")
    elif obj["leg"] != "rust" and obj["reflect_fallbacks"]:
        errors.append(f"row {i}: reflect_fallbacks must be empty except on the rust leg")
    results = obj["results"]
    if not isinstance(results, dict):
        errors.append(f"row {i}: results is not an object")
        return errors
    for q, r in results.items():
        if not isinstance(r, dict) or "pass" not in r or "value" not in r:
            errors.append(f"row {i}: result {q} must be {{pass, value}}")
            continue
        if r["pass"] not in (True, False, None):
            errors.append(f"row {i}: result {q} pass must be true, false, or null")
    return errors


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    with open(sys.argv[1]) as f:
        data = json.load(f)
    if not isinstance(data, list):
        print("top level must be a JSON array of report objects")
        sys.exit(1)

    all_errors = []
    for i, obj in enumerate(data):
        errs = check_object(i, obj)
        all_errors.extend(errs)
        if not errs:
            answered = sum(1 for r in obj["results"].values() if r["pass"] is not None)
            total = len(obj["results"])
            tag = f"{obj['os']} / {obj['browser'] or '-'} / {obj['provider']} [{obj['leg']}]"
            print(f"ok   {tag}: {answered}/{total} answered")

    if all_errors:
        print("\nERRORS:")
        for e in all_errors:
            print("  " + e)
        sys.exit(1)
    print(f"\nall {len(data)} objects well formed")


if __name__ == "__main__":
    main()
