#!/usr/bin/env python3
from __future__ import annotations

import sys
import subprocess
import json
from pathlib import Path

from replay_common import REPO_ROOT


def main() -> int:
    args = sys.argv[1:]
    fixture_paths = [arg for arg in args if Path(arg).name == "fixture.toml"]
    if len(fixture_paths) != 1:
        raise SystemExit(f"expected exactly one fixture.toml, found {len(fixture_paths)}")
    replay_args = args[:2] + fixture_paths
    completed = subprocess.run(
        [sys.executable, str(REPO_ROOT / "scripts/behavior/fixture3-g3rs-replay.py"), *replay_args],
        text=True,
        stdout=subprocess.PIPE,
        check=False,
    )
    if completed.returncode == 0:
        print(normalized_behavior_json(completed.stdout))
    return int(completed.returncode)


def normalized_behavior_json(raw_json: str) -> str:
    payload = json.loads(raw_json)
    for record in payload.get("records", []):
        record["fixture_id"] = "fixture-root"
        record.pop("fixture_hash", None)
    return json.dumps(payload, indent=2, sort_keys=True)


if __name__ == "__main__":
    sys.exit(main())
