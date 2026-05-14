#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

from replay_common import REPO_ROOT, load_fixture_metadata, load_toml, replay_record


SCHEMA_VERSION = "g3rs-replay-v1"


def main() -> int:
    manifest_path, fixture_metadata_paths = parse_args(sys.argv[1:])
    manifest = load_toml(manifest_path)
    fixture_set = manifest["fixture_set"]
    tool = fixture_set["tool"]
    records = []

    for fixture_metadata_path in fixture_metadata_paths:
        fixture_dir = fixture_metadata_path.parent
        fixture_id = fixture_dir.name
        metadata = load_fixture_metadata(fixture_dir)
        for index, argv in enumerate(metadata["commands"]):
            records.append(replay_record(tool, fixture_id, fixture_dir, metadata, index, argv))

    output = {
        "schema_version": SCHEMA_VERSION,
        "tool": tool,
        "manifest": manifest_path.relative_to(REPO_ROOT).as_posix(),
        "records": records,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0


def parse_args(argv: list[str]) -> tuple[Path, list[Path]]:
    if len(argv) < 3 or argv[0] != "--manifest":
        raise SystemExit("usage: fixture3-g3rs-replay.py --manifest <path> <fixture.toml>...")
    manifest_path = absolute_path(argv[1])
    fixture_paths = [absolute_path(path) for path in argv[2:]]
    for fixture_path in fixture_paths:
        if fixture_path.name != "fixture.toml":
            raise SystemExit(f"fixture path must end with fixture.toml: {fixture_path}")
    return manifest_path, fixture_paths


def absolute_path(raw_path: str) -> Path:
    path = Path(raw_path)
    return path if path.is_absolute() else REPO_ROOT / path


if __name__ == "__main__":
    sys.exit(main())
