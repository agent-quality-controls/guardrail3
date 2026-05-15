#!/usr/bin/env python3
"""Verify the G3RS init/validate CLI contract manifest."""

from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST = (
    REPO_ROOT
    / ".plans"
    / "2026-05-15-163150-g3rs-cli-contract-init-validate.md.manifest.toml"
)
CLI_MANIFEST = (
    REPO_ROOT
    / "apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml"
)


def load_manifest() -> dict:
    with MANIFEST.open("rb") as fp:
        return tomllib.load(fp)


def run_cli_help(argv: list[str]) -> tuple[int, str]:
    args = argv[1:]
    result = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            str(CLI_MANIFEST),
            "--",
            *args,
            "--help",
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    return result.returncode, result.stdout + result.stderr


def file_text(path: str) -> str:
    return (REPO_ROOT / path).read_text()


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for row in manifest.get("cli_command", []):
        argv = row["argv"]
        rc, output = run_cli_help(argv)
        must_parse = row["must_parse"]
        if must_parse and rc != 0:
            failures.append(f"cli should parse help: {' '.join(argv)}\n{output}")
        if not must_parse and rc == 0:
            failures.append(f"cli should reject: {' '.join(argv)}")

    root_rc, root_help = run_cli_help(["g3rs"])
    if root_rc != 0:
        failures.append(f"root help failed\n{root_help}")
    for row in manifest.get("root_help_phrase", []):
        if row["text"] not in root_help:
            failures.append(f"root help missing: {row['text']}")

    for row in manifest.get("doc", []):
        path = row["path"]
        doc_path = REPO_ROOT / path
        if not doc_path.exists():
            failures.append(f"doc missing: {path}")
            continue
        text = doc_path.read_text()
        for phrase in row["required_phrases"]:
            if phrase not in text:
                failures.append(f"doc {path} missing: {phrase}")

    all_rs = "\n".join(
        p.read_text(errors="ignore")
        for p in (REPO_ROOT / "apps/guardrail3-rs").rglob("*.rs")
    )
    all_hook_rs = "\n".join(
        p.read_text(errors="ignore")
        for p in (REPO_ROOT / "packages/rs/hooks").rglob("*.rs")
    )

    for row in manifest.get("managed_hook_file", []):
        if row["path"] not in all_rs:
            failures.append(f"managed hook path not referenced by CLI app: {row['path']}")
        if row["generated_header"] not in all_rs:
            failures.append(f"managed hook header not referenced by CLI app: {row['generated_header']}")

    for row in manifest.get("hook_chain", []):
        for key in ("pre_commit", "managed_hook", "required_repo_command", "required_workspace_command"):
            needle = row[key]
            if needle not in all_hook_rs and needle not in all_rs:
                failures.append(f"hook chain text not found in code: {needle}")

    report_text = file_text(
        "apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs"
    )
    for row in manifest.get("validate_output", []):
        for prefix in row["prefixes"]:
            if prefix.startswith("scope: "):
                needle = 'format!("scope: {scope}")'
            else:
                needle = prefix
            if needle not in report_text:
                failures.append(f"validate output prefix missing from renderer: {prefix}")

    if failures:
        print("g3rs-cli-contract: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("g3rs-cli-contract: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
