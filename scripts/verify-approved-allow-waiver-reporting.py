#!/usr/bin/env python3
"""Verify approved allow waiver reporting stays waiver-compatible."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path


RULE = "g3rs-cargo/approved-allow-inventory"
SELECTOR = "clippy:multiple_crate_versions"
EXPECTED_TITLE = "approved allow entry requires waiver"
FORBIDDEN_TITLE = "approved allow entry missing reason"


def main() -> int:
    repo = Path(__file__).resolve().parents[1]
    with tempfile.TemporaryDirectory() as temp_dir:
        fixture = Path(temp_dir)
        write_fixture(fixture)
        output = run_g3rs(repo, fixture)

    require(EXPECTED_TITLE in output, f"missing expected title: {EXPECTED_TITLE}")
    require(FORBIDDEN_TITLE not in output, f"forbidden title still present: {FORBIDDEN_TITLE}")
    require(
        f'waiver: rule="{RULE}" subject="Cargo.toml" selector="{SELECTOR}"' in output,
        "matching waiver line was not rendered",
    )
    return 0


def write_fixture(fixture: Path) -> None:
    (fixture / "Cargo.toml").write_text(
        """[package]
name = "approved-allow-fixture"
version = "0.1.0"
edition = "2024"

[lints.clippy]
multiple_crate_versions = "allow"
""",
        encoding="utf-8",
    )
    (fixture / "guardrail3-rs.toml").write_text(
        f"""profile = "library"

[[waivers]]
rule = "{RULE}"
subject = "Cargo.toml"
selector = "{SELECTOR}"
reason = "This fixture documents duplicate dependency allowance for regression coverage."
""",
        encoding="utf-8",
    )


def run_g3rs(repo: Path, fixture: Path) -> str:
    configured_bin = os.environ.get("G3RS_BIN")
    if configured_bin:
        command = [
            configured_bin,
            "validate",
            "workspace",
            "--path",
            str(fixture),
            "--family",
            "cargo",
            "--rules-only",
        ]
    else:
        command = [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            str(repo / "apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml"),
            "--",
            "validate",
            "workspace",
            "--path",
            str(fixture),
            "--family",
            "cargo",
            "--rules-only",
        ]

    completed = subprocess.run(
        command,
        cwd=repo,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    output = completed.stdout + completed.stderr
    return output


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


if __name__ == "__main__":
    sys.exit(main())
