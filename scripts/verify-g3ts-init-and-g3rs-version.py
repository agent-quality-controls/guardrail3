#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST = REPO_ROOT / ".plans/2026-05-18-151325-g3ts-init-and-g3rs-version.md.manifest.toml"
G3TS_MANIFEST = REPO_ROOT / "apps/guardrail3-ts/Cargo.toml"
G3RS_MANIFEST = REPO_ROOT / "apps/guardrail3-rs/Cargo.toml"
G3TS_BIN = REPO_ROOT / "apps/guardrail3-ts/target/debug/g3ts"
G3RS_BIN = REPO_ROOT / "apps/guardrail3-rs/target/debug/g3rs"


def main() -> int:
    failures: list[str] = []
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    build_bins(failures)
    verify_cli_commands(manifest, failures)
    verify_source_contains(manifest, failures)
    verify_source_forbidden(manifest, failures)
    verify_fixture_commands(manifest, failures)

    if failures:
        print("g3ts-init-and-g3rs-version: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("g3ts-init-and-g3rs-version: PASS")
    return 0


def build_bins(failures: list[str]) -> None:
    for manifest, package in ((G3TS_MANIFEST, "g3ts"), (G3RS_MANIFEST, "guardrail3-rs")):
        result = subprocess.run(
            [
                "cargo",
                "build",
                "--quiet",
                "--manifest-path",
                str(manifest),
                "-p",
                package,
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        if result.returncode != 0:
            failures.append(f"cargo build {package} failed: {result.stderr[-1200:]}")


def verify_cli_commands(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("cli_command", []):
        tool = row["tool"]
        argv = list(row["argv"])
        bin_path = G3TS_BIN if tool == "g3ts" else G3RS_BIN
        result = subprocess.run(
            [str(bin_path), *argv],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        expected = int(row["expect_exit"])
        if result.returncode != expected:
            failures.append(f"{tool} {' '.join(argv)} exited {result.returncode}, expected {expected}")
            continue
        output = result.stdout + result.stderr
        for needle in row.get("stdout_contains", []):
            if needle not in output:
                failures.append(f"{tool} {' '.join(argv)} output missing {needle!r}")


def verify_source_contains(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("source_contains", []):
        path = REPO_ROOT / row["path"]
        text = path.read_text(encoding="utf-8")
        for needle in row["required"]:
            if needle not in text:
                failures.append(f"{path.relative_to(REPO_ROOT)} missing {needle!r}")


def verify_source_forbidden(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("source_forbidden", []):
        path = REPO_ROOT / row["path"]
        text = path.read_text(encoding="utf-8")
        for needle in row["forbidden"]:
            if needle in text:
                failures.append(f"{path.relative_to(REPO_ROOT)} still contains {needle!r}")


def verify_fixture_commands(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("fixture_command", []):
        root = REPO_ROOT / row["path"]
        text = "\n".join(path.read_text(encoding="utf-8") for path in sorted(root.glob("**/fixture.toml")))
        for needle in row["required_commands"]:
            if needle not in text:
                failures.append(f"{root.relative_to(REPO_ROOT)} fixture commands missing {needle}")


if __name__ == "__main__":
    sys.exit(main())
