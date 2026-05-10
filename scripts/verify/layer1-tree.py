#!/usr/bin/env python3
"""Layer 1: tree shape. Each [[tree]] entry asserts a path (file or dir)
must exist or must NOT exist. Any drift is a FAIL.
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, emit, load_manifest, section


def main() -> int:
    manifest = load_manifest()
    entries = section(manifest, "tree")
    failures: list[str] = []
    for entry in entries:
        path = entry["path"]
        must_exist = entry["must_exist"]
        full = REPO_ROOT / path
        exists = full.exists()
        if must_exist and not exists:
            failures.append(f"MISSING (expected to exist): {path}")
        elif not must_exist and exists:
            failures.append(f"PRESENT (expected absent): {path}")
    if failures:
        print("layer:1-tree status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print(f"layer:1-tree status:PASS checks:{len(entries)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
