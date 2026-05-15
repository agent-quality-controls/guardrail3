#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

from replay_common import REPO_ROOT


SCHEMA_VERSION = "g3rs-report-output-v1"
TOOL = "guardrail3-rs-report-fixture-output"
G3RS_MANIFEST_PATH = REPO_ROOT / "apps" / "guardrail3-rs" / "Cargo.toml"


def main() -> int:
    _, fixture_paths = parse_args(sys.argv[1:])
    records = [render_record(path) for path in fixture_paths]
    output = {
        "records": records,
        "schema_version": SCHEMA_VERSION,
        "tool": TOOL,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0


def parse_args(argv: list[str]) -> tuple[Path, list[Path]]:
    if len(argv) < 3 or argv[0] != "--manifest":
        raise SystemExit("usage: fixture3-g3rs-report-output.py --manifest <path> <fixture.toml>...")
    manifest_path = absolute_path(argv[1])
    fixture_paths = [absolute_path(path) for path in argv[2:]]
    for fixture_path in fixture_paths:
        if fixture_path.name != "fixture.toml":
            raise SystemExit(f"fixture path must end with fixture.toml: {fixture_path}")
    return manifest_path, fixture_paths


def render_record(fixture_path: Path) -> dict[str, str]:
    result = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            str(G3RS_MANIFEST_PATH),
            "-p",
            "guardrail3-rs-report-assertions",
            "--bin",
            TOOL,
            "--",
            str(fixture_path),
        ],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise SystemExit(result.stderr)
    return {
        "fixture_id": fixture_path.parent.name,
        "rendered": result.stdout,
    }


def absolute_path(raw_path: str) -> Path:
    path = Path(raw_path)
    return path if path.is_absolute() else REPO_ROOT / path


if __name__ == "__main__":
    sys.exit(main())
