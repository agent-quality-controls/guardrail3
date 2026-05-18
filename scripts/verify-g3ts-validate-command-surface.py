#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
G3TS_MANIFEST = REPO_ROOT / "apps" / "guardrail3-ts" / "Cargo.toml"
G3TS_BIN = REPO_ROOT / "apps" / "guardrail3-ts" / "target" / "debug" / "g3ts"


def main() -> int:
    failures: list[str] = []
    build_g3ts(failures)
    verify_cli(failures)
    verify_source_text(failures)
    verify_fixture_commands(failures)

    if failures:
        print("g3ts-validate-command-surface: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("g3ts-validate-command-surface: PASS")
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
        failures.append(f"cargo build failed: {result.stderr[-1000:]}")


def verify_cli(failures: list[str]) -> None:
    expect_success(failures, ["--version"], ["g3ts "])
    expect_success(failures, ["--help"], ["init", "validate", "-V, --version"])
    expect_success(failures, ["init", "--help"], ["repo", "workspace"])
    expect_success(failures, ["init", "repo", "--help"], ["--path <PATH>", "--force"])
    expect_success(failures, ["init", "workspace", "--help"], ["--path <PATH>", "--force"])
    expect_success(failures, ["validate", "--help"], ["repo", "workspace"])
    expect_success(
        failures,
        ["validate", "workspace", "--help"],
        ["--path <PATH>", "--family <FAMILY>", "--rules-only"],
    )
    expect_success(
        failures,
        ["validate", "repo", "--help"],
        ["g3ts validate repo", "--path <PATH>"],
    )
    expect_failure(failures, ["validate-repo", "--help"])
    expect_failure(failures, ["validate", "--path", "."])


def verify_source_text(failures: list[str]) -> None:
    forbidden = {
        "docs/cli.md": ["g3ts validate-repo", "g3ts validate --path", "guardrail3 ts hooks-install"],
        "GUARDRAIL3_GUIDE.md": [
            "g3ts validate-repo",
            "g3ts validate --path",
            "guardrail3 ts hooks-install",
        ],
        "apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs": ["validate-repo"],
        "packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs": [
            "g3ts validate-repo",
            "g3ts validate --path <unit> --staged",
        ],
    }
    for rel_path, needles in forbidden.items():
        text = (REPO_ROOT / rel_path).read_text(encoding="utf-8")
        for needle in needles:
            if needle in text:
                failures.append(f"{rel_path}: forbidden text still present: {needle}")

    hook_source = (
        REPO_ROOT / "packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs"
    ).read_text(encoding="utf-8")
    for required in (
        "g3ts validate repo",
        "g3ts validate workspace --path <unit> --staged",
    ):
        if required not in hook_source:
            failures.append(f"hook source missing required text: {required}")
    init_source = (
        REPO_ROOT / "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs"
    ).read_text(encoding="utf-8")
    for required in (
        ".githooks/pre-commit.d/g3ts",
        'g3ts validate repo --path "$repo_root"',
        'g3ts validate workspace --path "$unit" --staged',
    ):
        if required not in init_source:
            failures.append(f"init source missing required text: {required}")


def verify_fixture_commands(failures: list[str]) -> None:
    for fixture in sorted((REPO_ROOT / "behavior/fixtures/g3ts-rule").glob("*/*/fixture.toml")):
        text = fixture.read_text(encoding="utf-8")
        if '["validate", "--path",' in text:
            failures.append(f"{fixture.relative_to(REPO_ROOT)}: uses removed workspace command")
    cli_fixture = REPO_ROOT / "behavior/fixtures/g3ts-cli-output/C10-help-contract/fixture.toml"
    cli_text = cli_fixture.read_text(encoding="utf-8")
    for required in (
        '["init", "--help"]',
        '["init", "repo", "--help"]',
        '["init", "workspace", "--help"]',
    ):
        if required not in cli_text:
            failures.append(f"{cli_fixture.relative_to(REPO_ROOT)}: missing {required}")
    repo_fixture = REPO_ROOT / "behavior/fixtures/g3ts-validate-repo/R00-invalid-repo-root/fixture.toml"
    if '["validate", "repo", "--path",' not in repo_fixture.read_text(encoding="utf-8"):
        failures.append(f"{repo_fixture.relative_to(REPO_ROOT)}: does not use validate repo")


def expect_success(failures: list[str], argv: list[str], required_text: list[str]) -> None:
    result = run_g3ts(argv)
    if result.returncode != 0:
        failures.append(f"g3ts {' '.join(argv)}: expected success, got {result.returncode}")
        return
    output = result.stdout + result.stderr
    for needle in required_text:
        if needle not in output:
            failures.append(f"g3ts {' '.join(argv)}: output missing {needle}")


def expect_failure(failures: list[str], argv: list[str]) -> None:
    result = run_g3ts(argv)
    if result.returncode == 0:
        failures.append(f"g3ts {' '.join(argv)}: expected failure, got success")


def run_g3ts(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(G3TS_BIN), *argv],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


if __name__ == "__main__":
    sys.exit(main())
