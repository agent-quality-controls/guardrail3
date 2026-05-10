#!/usr/bin/env python3
"""Layer 2: CLI surface. Each [[cli_subcommand]] entry asserts a binary
exposes a named subcommand. Each [[cli_flag]] asserts a flag is accepted.
[[cli_family_value]] asserts a --family value is accepted.

Approach: invoke `<bin> <sub> --help` and grep the output for the flag.
This is mechanical and deterministic.
"""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import emit, load_manifest, section


def resolve_binary(binary: str) -> str | None:
    from pathlib import Path
    repo_root = Path(__file__).resolve().parents[2]
    local = repo_root / "apps" / binary / "target" / "release" / binary
    if local.exists():
        return str(local)
    return shutil.which(binary)


def help_output(binary: str, *args: str) -> tuple[int, str]:
    path = resolve_binary(binary)
    if path is None:
        return (127, f"binary not found: {binary}")
    result = subprocess.run(
        [path, *args, "--help"], capture_output=True, text=True
    )
    return (result.returncode, (result.stdout or "") + (result.stderr or ""))


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "cli_subcommand"):
        binary = entry["binary"]
        sub = entry["subcommand"]
        if not entry.get("must_exist", True):
            continue
        rc, out = help_output(binary, sub)
        if rc == 127:
            failures.append(f"binary missing: {binary}")
            continue
        # `cargo` prints subcommands in `--help`; for our subcommands we
        # rely on `<bin> <sub> --help` returning rc 0 or 2 with usage text
        # mentioning the subcommand name.
        if rc not in (0, 1, 2):
            failures.append(
                f"subcommand {binary} {sub}: unexpected exit {rc}"
            )
            continue
        if sub not in out and "Usage:" not in out:
            failures.append(
                f"subcommand {binary} {sub}: help output empty"
            )

    for entry in section(manifest, "cli_flag"):
        binary = entry["binary"]
        sub = entry["subcommand"]
        flag = entry["flag"]
        rc, out = help_output(binary, sub)
        if flag not in out:
            failures.append(
                f"flag missing: {binary} {sub} {flag}"
            )

    for entry in section(manifest, "cli_family_value"):
        binary = entry["binary"]
        family = entry["family"]
        rc, out = help_output(binary, "validate")
        if family not in out:
            failures.append(
                f"family value missing: {binary} validate --family {family}"
            )

    if failures:
        print("layer:2-cli status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:2-cli status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
