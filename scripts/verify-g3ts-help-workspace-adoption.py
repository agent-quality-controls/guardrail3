#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST = REPO_ROOT / ".plans/2026-05-19-182155-g3ts-help-workspace-adoption.md.manifest.toml"
G3TS_MANIFEST = REPO_ROOT / "apps/guardrail3-ts/Cargo.toml"
G3TS_BIN = REPO_ROOT / "apps/guardrail3-ts/target/debug/g3ts"


def main() -> int:
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    failures: list[str] = []
    build_g3ts(failures)
    verify_cli_help(manifest, failures)
    verify_source(manifest, failures)
    if failures:
        print("FAIL g3ts help workspace adoption manifest")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("PASS g3ts help workspace adoption manifest")
    return 0


def build_g3ts(failures: list[str]) -> None:
    result = subprocess.run(
        [
            "cargo",
            "build",
            "--quiet",
            "--manifest-path",
            str(G3TS_MANIFEST),
            "-p",
            "g3ts",
            "--bin",
            "g3ts",
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        failures.append(f"cargo build failed: {result.stderr[-2000:]}")


def verify_cli_help(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("cli_help_contains", []):
        command = row["command"]
        expected = row["text"]
        result = subprocess.run(
            [str(G3TS_BIN), *command],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        if result.returncode != 0:
            failures.append(f"g3ts {' '.join(command)} exited {result.returncode}")
            continue
        output = result.stdout + result.stderr
        if expected not in output:
            failures.append(f"g3ts {' '.join(command)} missing text: {expected}")


def verify_source(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("source_contains", []):
        path = REPO_ROOT / row["path"]
        expected = row["text"]
        text = path.read_text(encoding="utf-8")
        if expected not in text:
            failures.append(f"{path.relative_to(REPO_ROOT)} missing text: {expected}")


if __name__ == "__main__":
    sys.exit(main())
