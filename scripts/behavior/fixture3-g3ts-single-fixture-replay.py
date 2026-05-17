#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
REPLAY = REPO_ROOT / "scripts" / "behavior" / "fixture3-g3ts-fixture-replay.py"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--fixture-id", required=True)
    parser.add_argument("paths", nargs="+")
    args = parser.parse_args()

    fixture_tomls = sorted({Path(path) for path in args.paths if Path(path).name == "fixture.toml"})
    if len(fixture_tomls) != 1:
        print(
            f"expected exactly one fixture.toml in reducer trial, got {len(fixture_tomls)}",
            file=sys.stderr,
        )
        return 1

    completed = subprocess.run(
        [
            sys.executable,
            str(REPLAY),
            "--manifest",
            args.manifest,
            str(fixture_tomls[0]),
        ],
        cwd=REPO_ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.stderr:
        print(completed.stderr, file=sys.stderr, end="")
    if completed.returncode != 0:
        print(completed.stdout, end="")
        return completed.returncode
    data = json.loads(completed.stdout)
    for record in data.get("records", []):
        if isinstance(record, dict):
            record["fixture_id"] = args.fixture_id
    print(json.dumps(data, indent=2, sort_keys=True))
    return completed.returncode


if __name__ == "__main__":
    sys.exit(main())
