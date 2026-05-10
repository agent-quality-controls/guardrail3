#!/usr/bin/env python3
"""Layer 3c: hook required and forbidden patterns.

Reads .githooks/pre-commit and asserts:
- every [[hook_required]] pattern appears.
- no [[hook_forbidden]] pattern appears (literal substring).
- no [[hook_forbidden_invocation]] command appears as a word-bounded
  command invocation, except where the manifest declares an
  `allowed_context` and that exact context is the only occurrence.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, load_manifest, section


HOOK_PATH = REPO_ROOT / ".githooks" / "pre-commit"


def main() -> int:
    if not HOOK_PATH.exists():
        print("layer:3c-hook-patterns status:FAIL")
        print("  hook missing: .githooks/pre-commit")
        return 1
    text = HOOK_PATH.read_text()
    failures: list[str] = []
    manifest = load_manifest()

    for entry in section(manifest, "hook_required"):
        pattern = entry["pattern"]
        if pattern not in text:
            failures.append(
                f"required pattern missing: {pattern!r} ({entry.get('description', '')})"
            )

    for entry in section(manifest, "hook_forbidden"):
        pattern = entry["pattern"]
        if pattern in text:
            failures.append(
                f"forbidden pattern present: {pattern!r} ({entry.get('description', '')})"
            )

    for entry in section(manifest, "hook_forbidden_invocation"):
        cmd = entry["command"]
        allowed_contexts: list[str] = entry.get("allowed_contexts") or (
            [entry["allowed_context"]] if entry.get("allowed_context") else []
        )
        # Look for the command as a word-bounded invocation.
        word_pattern = re.compile(rf"(?<![A-Za-z0-9_])({re.escape(cmd)})\s")
        all_hits = [
            (m.start(), m.group(0)) for m in word_pattern.finditer(text)
        ]
        if not all_hits:
            continue
        if not allowed_contexts:
            failures.append(
                f"forbidden invocation present: {cmd} (no allowed context)"
            )
            continue
        bad: list[str] = []
        for pos, _ in all_hits:
            longest_ctx = max((len(c) for c in allowed_contexts), default=0)
            window_start = max(0, pos - 10)
            window_end = min(len(text), pos + longest_ctx + 20)
            window = text[window_start:window_end]
            if not any(ctx in window for ctx in allowed_contexts):
                bad.append(text[pos : pos + 60].splitlines()[0])
        if bad:
            failures.append(
                f"forbidden invocation outside allowed_contexts {allowed_contexts!r}: {cmd}: {bad[0]}"
            )

    if failures:
        print("layer:3c-hook-patterns status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:3c-hook-patterns status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
