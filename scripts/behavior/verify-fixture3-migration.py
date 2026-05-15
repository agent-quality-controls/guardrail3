#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import yaml

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = (
    REPO_ROOT / ".plans" / "2026-05-14-113549-migrate-behavior-replay-to-fixture3.md.manifest.toml"
)


def main() -> int:
    manifest = load_toml(MANIFEST_PATH)
    failures: list[str] = []
    failures.extend(verify_tool())
    failures.extend(verify_renames(manifest.get("rename", [])))
    failures.extend(verify_forbidden_paths(manifest.get("forbidden_path", [])))
    failures.extend(verify_suites(manifest.get("fixture3_suite", [])))
    failures.extend(verify_harness_outputs(manifest.get("fixture3_suite", [])))
    failures.extend(verify_active_references(manifest.get("forbidden_active_reference", [])))

    if failures:
        print("fixture3-migration: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("fixture3-migration: PASS")
    return 0


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_tool() -> list[str]:
    failures: list[str] = []
    version = subprocess.run(
        ["fixture3", "--version"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if version.returncode != 0 or not version.stdout.strip().startswith("fixture3 "):
        failures.append(f"fixture3 version unavailable: {version.stdout.strip()!r}")
    help_result = subprocess.run(
        ["fixture3", "check", "--help"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    for required in ("--suite", "--all", "--manifest"):
        if required not in help_result.stdout:
            failures.append(f"fixture3 check help missing {required}")
    return failures


def verify_renames(rows: list[dict[str, str]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        if (REPO_ROOT / row["from"]).exists():
            failures.append(f"old path still exists: {row['from']}")
        if not (REPO_ROOT / row["to"]).exists():
            failures.append(f"new path missing: {row['to']}")
    return failures


def verify_forbidden_paths(rows: list[dict[str, str]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        if (REPO_ROOT / row["path"]).exists():
            failures.append(f"forbidden path exists: {row['path']}")
    return failures


def verify_suites(rows: list[dict[str, str]]) -> list[str]:
    failures: list[str] = []
    manifest = load_yaml(REPO_ROOT / "fixture3.yaml")
    suites = manifest.get("suites", {})
    if not isinstance(suites, dict):
        return ["fixture3.yaml must define suites"]

    for row in rows:
        suite = suites.get(row["name"])
        if not isinstance(suite, dict):
            failures.append(f"fixture3 suite missing: {row['name']}")
            continue
        command = suite.get("command", {})
        argv = command.get("argv", [])
        if row["runner"] not in argv:
            failures.append(f"{row['name']}: runner mismatch")
        storage = suite.get("storage", {})
        for key in ("approved_dir", "received_dir", "diff_dir"):
            if storage.get(key) != row[key]:
                failures.append(f"{row['name']}: {key} mismatch")
    return failures


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as file:
        loaded = yaml.safe_load(file)
    if not isinstance(loaded, dict):
        raise ValueError(f"{path} did not parse to a mapping")
    return loaded


def verify_harness_outputs(rows: list[dict[str, str]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        approved = REPO_ROOT / row["approved_dir"] / "approved.normalized.json"
        meta = REPO_ROOT / row["approved_dir"] / "approved.meta.json"
        if not approved.is_file():
            failures.append(f"{row['name']}: approved output missing")
            continue
        if not meta.is_file():
            failures.append(f"{row['name']}: approved metadata missing")
        data = json.loads(approved.read_text(encoding="utf-8"))
        expected_schema_version = row.get("schema_version", "g3rs-replay-v1")
        if data.get("schema_version") != expected_schema_version:
            failures.append(f"{row['name']}: schema_version mismatch")
        records = data.get("records")
        if not isinstance(records, list) or not records:
            failures.append(f"{row['name']}: records missing")
    return failures


def verify_active_references(rows: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        pattern = row["pattern"]
        for file_path in matched_files(row["path_globs"]):
            text = file_path.read_text(encoding="utf-8", errors="ignore")
            if pattern in text:
                failures.append(f"{file_path.relative_to(REPO_ROOT)}: forbidden active reference {pattern}")
    return failures


def matched_files(globs: list[str]) -> list[Path]:
    files: list[Path] = []
    for glob in globs:
        files.extend(path for path in REPO_ROOT.glob(glob) if path.is_file())
    return sorted(set(files))


if __name__ == "__main__":
    sys.exit(main())
