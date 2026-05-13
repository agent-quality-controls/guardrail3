#!/usr/bin/env python3
from __future__ import annotations

import sys
import json
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore

from replay_common import REPO_ROOT


COMPACTION_MANIFEST = (
    REPO_ROOT / ".plans" / "2026-05-13-004723-g3rs-behavior-fixture-compaction.md.manifest.toml"
)
BEHAVIOR_MANIFEST = (
    REPO_ROOT / ".plans" / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
)


def load_toml(path: Path) -> dict:
    with path.open("rb") as file:
        return tomllib.load(file)


def golden_records_by_fixture(path: Path) -> dict[str, dict]:
    if not path.is_file():
        return {}
    data = json.loads(path.read_text(encoding="utf-8"))
    records: dict[str, dict] = {}
    for record in data.get("records", []):
        fixture_id = record.get("fixture_id")
        command_index = record.get("command_index")
        if isinstance(fixture_id, str) and command_index == 0:
            records[fixture_id] = record
    return records


def main() -> int:
    compaction = load_toml(COMPACTION_MANIFEST)
    behavior = load_toml(BEHAVIOR_MANIFEST)
    fixture_root = REPO_ROOT / compaction["fixture_set"]["root"]
    golden_records = golden_records_by_fixture(REPO_ROOT / compaction["fixture_set"]["golden_output"])
    active_ids = {entry["id"] for entry in behavior["fixture"]}
    kept_ids = {entry["id"] for entry in compaction.get("kept_fixture", [])}
    removed = compaction.get("removed_fixture", [])
    removed_ids = {entry["id"] for entry in removed}
    failures: list[str] = []

    for fixture_id in kept_ids:
        if fixture_id not in active_ids:
            failures.append(f"{fixture_id}: kept fixture missing from behavior manifest")
        if not (fixture_root / fixture_id).is_dir():
            failures.append(f"{fixture_id}: kept fixture directory missing")
        if fixture_id not in golden_records:
            failures.append(f"{fixture_id}: kept fixture approved golden record missing")

    for entry in removed:
        fixture_id = entry["id"]
        replacement = entry["replacement"]
        if fixture_id in active_ids:
            failures.append(f"{fixture_id}: removed fixture still active in behavior manifest")
        if (fixture_root / fixture_id).exists():
            failures.append(f"{fixture_id}: removed fixture directory still exists")
        if fixture_id in golden_records:
            failures.append(f"{fixture_id}: removed fixture still has approved golden record")
        if replacement not in kept_ids and replacement != "removed-as-topology-pollution":
            failures.append(f"{fixture_id}: replacement is not a kept fixture: {replacement}")

    for entry in compaction.get("kept_fixture", []):
        fixture_id = entry["id"]
        for source_id in entry.get("merged_from", []):
            if source_id not in removed_ids:
                failures.append(f"{fixture_id}: merged source is not listed as removed: {source_id}")
        record = golden_records.get(fixture_id)
        if record is None:
            continue
        stdout = record.get("stdout", "")
        if not isinstance(stdout, str):
            failures.append(f"{fixture_id}: baseline stdout must be a string")
            continue
        finding_lines = [
            line for line in stdout.splitlines() if line.startswith("[Error]") or line.startswith("[Warn]")
        ]
        for forbidden in entry.get("must_not_emit", []):
            for line in finding_lines:
                if forbidden in line:
                    failures.append(f"{fixture_id}: forbidden finding emitted: {line}")

    if failures:
        print("behavior-compaction: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"behavior-compaction: PASS kept:{len(kept_ids)} removed:{len(removed_ids)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
