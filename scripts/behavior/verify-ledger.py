#!/usr/bin/env python3
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-ledger.toml"
DEFAULT_MANIFEST_PATH = (
    REPO_ROOT
    / ".plans"
    / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
)

ALLOWED_GROUP_STATUSES = {"active", "complete"}
ALLOWED_KINDS = {
    "behavior",
    "compile_contract",
    "obsolete",
    "private_implementation_only",
    "replay_system",
    "unclassified",
}
ALLOWED_STATUSES = {
    "deleted_obsolete",
    "deleted_private_implementation",
    "kept_compile_contract",
    "kept_replay_system",
    "migrated_deleted",
    "unclassified",
}
TERMINAL_STATUS_BY_KIND = {
    "behavior": "migrated_deleted",
    "compile_contract": "kept_compile_contract",
    "obsolete": "deleted_obsolete",
    "private_implementation_only": "deleted_private_implementation",
    "replay_system": "kept_replay_system",
}


def main() -> int:
    ledger_path, manifest_path = paths_from_argv(sys.argv[1:])
    ledger = load_toml(ledger_path)
    manifest = load_toml(manifest_path)
    failures = verify_ledger(ledger, manifest)
    if failures:
        print("behavior-ledger: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    groups = ledger.get("group", [])
    tests = ledger.get("test", [])
    print(f"behavior-ledger: PASS groups:{len(groups)} tests:{len(tests)}")
    return 0


def paths_from_argv(argv: list[str]) -> tuple[Path, Path]:
    ledger_path = DEFAULT_LEDGER_PATH
    manifest_path = DEFAULT_MANIFEST_PATH
    index = 0
    while index < len(argv):
        arg = argv[index]
        if arg == "--ledger" and index + 1 < len(argv):
            ledger_path = absolute_path(argv[index + 1])
            index += 2
            continue
        if arg == "--manifest" and index + 1 < len(argv):
            manifest_path = absolute_path(argv[index + 1])
            index += 2
            continue
        raise SystemExit("usage: verify-ledger.py [--ledger <path>] [--manifest <path>]")
    return ledger_path, manifest_path


def absolute_path(raw_path: str) -> Path:
    path = Path(raw_path)
    return path if path.is_absolute() else REPO_ROOT / path


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_ledger(ledger: dict[str, Any], manifest: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    groups = ledger.get("group", [])
    tests = ledger.get("test", [])
    if not isinstance(groups, list) or not groups:
        return ["ledger must define at least one [[group]]"]
    if not isinstance(tests, list):
        failures.append("ledger [[test]] entries must be a list")
        tests = []
    fixture_ids = fixture_ids_from_manifest(manifest, failures)
    group_roots = group_roots_from_ledger(groups, failures)
    row_paths: list[str] = []

    for row_index, row in enumerate(tests):
        if not isinstance(row, dict):
            failures.append(f"test row {row_index}: row must be a table")
            continue
        old_test_path = row.get("old_test_path")
        kind = row.get("kind")
        status = row.get("status")
        fixture = row.get("fixture", "")
        if not isinstance(old_test_path, str) or not old_test_path:
            failures.append(f"test row {row_index}: old_test_path must be a non-empty string")
            continue
        row_paths.append(old_test_path)
        failures.extend(verify_repo_relative_path(old_test_path, f"test row {row_index}"))
        if not path_belongs_to_active_group(old_test_path, group_roots):
            failures.append(f"{old_test_path}: row is outside declared active groups")
        if kind not in ALLOWED_KINDS:
            failures.append(f"{old_test_path}: invalid kind {kind}")
        if status not in ALLOWED_STATUSES:
            failures.append(f"{old_test_path}: invalid status {status}")
        if kind == "unclassified" and status != "unclassified":
            failures.append(f"{old_test_path}: unclassified kind must use unclassified status")
        if kind in TERMINAL_STATUS_BY_KIND and status != TERMINAL_STATUS_BY_KIND[kind]:
            failures.append(
                f"{old_test_path}: {kind} row must use status {TERMINAL_STATUS_BY_KIND[kind]}"
            )
        if kind == "behavior" and not fixture:
            failures.append(f"{old_test_path}: behavior rows must name a replay fixture")
        if fixture and fixture not in fixture_ids:
            failures.append(f"{old_test_path}: unknown fixture {fixture}")

    duplicates = sorted(path for path, count in Counter(row_paths).items() if count > 1)
    for duplicate in duplicates:
        failures.append(f"{duplicate}: duplicate ledger row")

    for group in groups:
        if not isinstance(group, dict):
            failures.append("group row must be a table")
            continue
        root = group.get("root")
        group_id = group.get("id")
        status = group.get("status")
        if not isinstance(root, str) or not root:
            failures.append(f"group {group_id}: root must be a non-empty string")
            continue
        failures.extend(verify_repo_relative_path(root, f"group {group_id} root"))
        if status == "complete":
            for row in tests:
                if isinstance(row, dict) and row_path_under_root(row.get("old_test_path"), root):
                    if row.get("status") == "unclassified":
                        failures.append(f"{row['old_test_path']}: complete group cannot contain unclassified rows")
        for test_file in discover_test_files(REPO_ROOT / root):
            rel_path = test_file.relative_to(REPO_ROOT).as_posix()
            if row_paths.count(rel_path) != 1:
                failures.append(f"{rel_path}: expected exactly one ledger row")
    return failures


def fixture_ids_from_manifest(manifest: dict[str, Any], failures: list[str]) -> set[str]:
    fixture_rows = manifest.get("fixture", [])
    if not isinstance(fixture_rows, list):
        failures.append("manifest fixture rows must be a list")
        return set()
    fixture_ids: set[str] = set()
    for row in fixture_rows:
        if isinstance(row, dict) and isinstance(row.get("id"), str):
            fixture_ids.add(row["id"])
    return fixture_ids


def group_roots_from_ledger(groups: list[Any], failures: list[str]) -> list[str]:
    roots: list[str] = []
    seen_ids: set[str] = set()
    for group in groups:
        if not isinstance(group, dict):
            failures.append("group row must be a table")
            continue
        group_id = group.get("id")
        root = group.get("root")
        status = group.get("status")
        if not isinstance(group_id, str) or not group_id:
            failures.append("group id must be a non-empty string")
        elif group_id in seen_ids:
            failures.append(f"group {group_id}: duplicate group id")
        else:
            seen_ids.add(group_id)
        if status not in ALLOWED_GROUP_STATUSES:
            failures.append(f"group {group_id}: invalid status {status}")
        if isinstance(root, str) and root:
            roots.append(root.rstrip("/"))
    return roots


def verify_repo_relative_path(path: str, label: str) -> list[str]:
    candidate = Path(path)
    if candidate.is_absolute() or ".." in candidate.parts:
        return [f"{label}: path must be repo-relative and must not escape repo"]
    return []


def path_belongs_to_active_group(path: str, group_roots: list[str]) -> bool:
    return any(row_path_under_root(path, root) for root in group_roots)


def row_path_under_root(path: Any, root: str) -> bool:
    if not isinstance(path, str):
        return False
    normalized_root = root.rstrip("/")
    return path == normalized_root or path.startswith(f"{normalized_root}/")


def discover_test_files(root: Path) -> list[Path]:
    if not root.is_dir():
        return []
    files: list[Path] = []
    for path in root.rglob("*.rs"):
        rel_parts = path.relative_to(root).parts
        if any(part in {"target", ".cargo-target"} for part in rel_parts):
            continue
        if is_rust_test_file(rel_parts):
            files.append(path)
    return sorted(files)


def is_rust_test_file(rel_parts: tuple[str, ...]) -> bool:
    name = rel_parts[-1]
    if name.endswith("_test.rs") or name.endswith("_tests.rs"):
        return True
    if "tests" in rel_parts:
        return True
    return any(part.endswith("_tests") for part in rel_parts[:-1])


if __name__ == "__main__":
    sys.exit(main())
