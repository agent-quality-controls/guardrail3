#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = (
    REPO_ROOT
    / ".plans"
    / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
)
PRE_COMMIT_HOOK_PATH = REPO_ROOT / ".githooks" / "pre-commit"


def load_toml(path: Path) -> dict:
    with path.open("rb") as file:
        return tomllib.load(file)


def relative_files(root: Path) -> list[str]:
    files: list[str] = []
    for path in root.rglob("*"):
        if path.is_file() or path.is_symlink():
            files.append(path.relative_to(root).as_posix())
    return sorted(files)


def main() -> int:
    manifest = load_toml(MANIFEST_PATH)
    fixture_set = manifest["fixture_set"]
    fixture_root = REPO_ROOT / fixture_set["root"]
    failures: list[str] = []

    if not fixture_root.is_dir():
        failures.append(f"missing fixture root: {fixture_set['root']}")

    if not PRE_COMMIT_HOOK_PATH.is_file():
        failures.append("missing pre-commit hook")
    else:
        hook = PRE_COMMIT_HOOK_PATH.read_text(encoding="utf-8")
        required_hook_fragments = [
            "VALIDATION_STAGED_FILES=",
            "grep -vE '^behavior/fixtures/'",
            'MIGRATION_FILES=$(echo "$VALIDATION_STAGED_FILES"',
            'STAGED_PACKAGE_JSON=$(echo "$VALIDATION_STAGED_FILES"',
            'RUST_RELEVANT_FILES=$(echo "$VALIDATION_STAGED_FILES"',
            'TS_RELEVANT_FILES=$(echo "$VALIDATION_STAGED_FILES"',
        ]
        for fragment in required_hook_fragments:
            if fragment not in hook:
                failures.append(f"pre-commit hook does not exclude behavior fixtures from validation routing: {fragment}")

    for link in fixture_set["required_shared_links"]:
        path = REPO_ROOT / link
        if not path.is_symlink():
            failures.append(f"missing required symlink: {link}")
        elif not path.exists():
            failures.append(f"broken required symlink: {link}")

    expected_ids = [entry["id"] for entry in manifest["fixture"]]
    actual_ids = sorted(path.name for path in fixture_root.iterdir() if path.is_dir()) if fixture_root.exists() else []
    if actual_ids != sorted(expected_ids):
        failures.append(f"fixture directory set mismatch: expected {sorted(expected_ids)}, got {actual_ids}")

    for entry in manifest["fixture"]:
        fixture_id = entry["id"]
        fixture_dir = fixture_root / fixture_id
        metadata_path = fixture_dir / "fixture.toml"
        if not metadata_path.is_file():
            failures.append(f"{fixture_id}: missing fixture.toml")
            continue

        metadata = load_toml(metadata_path)
        for key in ("id", "tool", "expected_exit"):
            if key not in metadata:
                failures.append(f"{fixture_id}: missing metadata key {key}")
        if metadata.get("id") != fixture_id:
            failures.append(f"{fixture_id}: fixture.toml id mismatch: {metadata.get('id')}")
        if metadata.get("tool") != fixture_set["tool"]:
            failures.append(f"{fixture_id}: fixture.toml tool mismatch: {metadata.get('tool')}")
        if metadata.get("expected_exit") != entry["expected_exit"]:
            failures.append(
                f"{fixture_id}: expected_exit mismatch: manifest {entry['expected_exit']} metadata {metadata.get('expected_exit')}"
            )
        if entry.get("runner_mode") and metadata.get("runner_mode") != entry["runner_mode"]:
            failures.append(f"{fixture_id}: runner_mode mismatch")

        repo_dir = fixture_dir / "repo"
        if not repo_dir.exists():
            failures.append(f"{fixture_id}: missing repo directory")

        if entry.get("closed_file_list"):
            expected_files = sorted(entry["files"])
            actual_files = relative_files(fixture_dir)
            if actual_files != expected_files:
                failures.append(f"{fixture_id}: file list mismatch: expected {expected_files}, got {actual_files}")
        else:
            for required in entry.get("required_files", []):
                if not (fixture_dir / required).is_file():
                    failures.append(f"{fixture_id}: missing required file {required}")
            for forbidden in entry.get("forbidden_paths", []):
                if (fixture_dir / forbidden).exists():
                    failures.append(f"{fixture_id}: forbidden path exists {forbidden}")

    if failures:
        print("behavior-fixtures: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"behavior-fixtures: PASS fixtures:{len(expected_ids)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
