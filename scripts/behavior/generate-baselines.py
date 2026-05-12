#!/usr/bin/env python3
from __future__ import annotations

import sys
from datetime import UTC, datetime

from baseline_common import (
    REPO_ROOT,
    baseline_path,
    load_fixture_metadata,
    load_manifest,
    output_record,
    write_json,
)


def main() -> int:
    manifest = load_manifest()
    fixture_set = manifest["fixture_set"]
    fixture_root = REPO_ROOT / fixture_set["root"]
    baseline_root = REPO_ROOT / fixture_set["baseline_root"]
    tool = fixture_set["tool"]
    written = 0

    for entry in manifest["fixture"]:
        if not entry.get("baseline_required"):
            continue
        fixture_id = entry["id"]
        fixture_dir = fixture_root / fixture_id
        metadata = load_fixture_metadata(fixture_dir)
        for index, argv in enumerate(metadata["commands"]):
            record = output_record(tool, fixture_id, fixture_dir, metadata, index, argv)
            record["created_at"] = datetime.now(UTC).isoformat()
            write_json(baseline_path(baseline_root, fixture_id, index), record)
            written += 1

    print(f"behavior-baselines: WROTE records:{written}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
