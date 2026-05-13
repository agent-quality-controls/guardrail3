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
    REPO_ROOT
    / ".plans"
    / "2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md.manifest.toml"
)


def main() -> int:
    manifest = load_toml(MANIFEST_PATH)
    failures: list[str] = []
    failures.extend(verify_tool(manifest["tool"]))
    failures.extend(verify_file_rows(manifest))
    failures.extend(verify_yaml_suites(manifest.get("yaml_suite", [])))
    failures.extend(verify_harness_outputs(manifest.get("harness_output", [])))
    failures.extend(verify_golden_outputs(manifest.get("golden_output", [])))
    failures.extend(verify_script_text(manifest))
    failures.extend(verify_forbidden_active_references(manifest.get("forbidden_active_reference", [])))

    if failures:
        print("goldencheck-migration: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("goldencheck-migration: PASS")
    return 0


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_tool(row: dict[str, str]) -> list[str]:
    failures: list[str] = []
    version = subprocess.run(
        [row["required_name"], "--version"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    expected = f"{row['required_name']} {row['required_version']}"
    if version.returncode != 0 or version.stdout.strip() != expected:
        failures.append(f"tool version mismatch: expected {expected!r}, got {version.stdout.strip()!r}")
    help_result = subprocess.run(
        [row["required_name"], "check", "--help"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if row["required_check_mode"] not in help_result.stdout:
        failures.append(f"goldencheck check help missing {row['required_check_mode']}")
    return failures


def verify_file_rows(manifest: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    for row in manifest.get("file_exists", []):
        path = REPO_ROOT / row["path"]
        if not path.exists():
            failures.append(f"missing path: {row['path']}")
    for row in manifest.get("absent_path", []):
        path = REPO_ROOT / row["path"]
        if path.exists():
            failures.append(f"path must be absent: {row['path']}")
    return failures


def verify_yaml_suites(rows: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    by_manifest: dict[str, dict[str, Any]] = {}
    for row in rows:
        manifest_path = row["manifest"]
        if manifest_path not in by_manifest:
            with (REPO_ROOT / manifest_path).open("r", encoding="utf-8") as file:
                by_manifest[manifest_path] = yaml.safe_load(file)
        suite = by_manifest[manifest_path].get("suites", {}).get(row["name"])
        if not isinstance(suite, dict):
            failures.append(f"goldencheck suite missing: {row['name']}")
            continue
        expected_fixture_globs = [row["fixture_glob"]]
        if suite.get("fixtures") != expected_fixture_globs:
            failures.append(f"{row['name']}: fixture globs mismatch")
        if suite.get("command", {}).get("argv") != row["command_argv"]:
            failures.append(f"{row['name']}: command argv mismatch")
        if suite.get("command", {}).get("ok_exit_codes") != row["ok_exit_codes"]:
            failures.append(f"{row['name']}: ok_exit_codes mismatch")
        if suite.get("output", {}).get("format") != row["output_format"]:
            failures.append(f"{row['name']}: output format mismatch")
        storage = suite.get("storage", {})
        for manifest_key, yaml_key in (
            ("approved_dir", "approved_dir"),
            ("received_dir", "received_dir"),
            ("diff_dir", "diff_dir"),
        ):
            if storage.get(yaml_key) != row[manifest_key]:
                failures.append(f"{row['name']}: storage {yaml_key} mismatch")
    return failures


def verify_harness_outputs(rows: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        script = REPO_ROOT / row["script"]
        if not script.is_file():
            failures.append(f"harness missing: {row['script']}")
            continue
        for golden_path in (
            REPO_ROOT / "behavior/golden/g3rs-validate/approved.normalized.json",
            REPO_ROOT / "behavior/golden/g3rs-validate-repo/approved.normalized.json",
        ):
            if not golden_path.is_file():
                continue
            data = json.loads(golden_path.read_text(encoding="utf-8"))
            if data.get("schema_version") != row["schema_version"]:
                failures.append(f"{golden_path.relative_to(REPO_ROOT)}: schema_version mismatch")
            records = data.get(row["top_level_array"])
            if not isinstance(records, list) or not records:
                failures.append(f"{golden_path.relative_to(REPO_ROOT)}: records missing")
                continue
            for index, record in enumerate(records):
                for field in row["required_record_fields"]:
                    if field not in record:
                        failures.append(f"{golden_path.relative_to(REPO_ROOT)} record {index}: missing {field}")
                for field in row["forbidden_record_fields"]:
                    if field in record:
                        failures.append(f"{golden_path.relative_to(REPO_ROOT)} record {index}: forbidden {field}")
    return failures


def verify_golden_outputs(rows: list[dict[str, str]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        for key in ("approved_normalized", "approved_meta"):
            path = REPO_ROOT / row[key]
            if not path.is_file():
                failures.append(f"{row['suite']}: missing {key} {row[key]}")
    return failures


def verify_script_text(manifest: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    for row in manifest.get("script_contains", []):
        text = (REPO_ROOT / row["path"]).read_text(encoding="utf-8")
        if row["required"] not in text:
            failures.append(f"{row['path']}: missing {row['required']!r}")
    for row in manifest.get("script_not_contains", []):
        text = (REPO_ROOT / row["path"]).read_text(encoding="utf-8")
        if row["forbidden"] in text:
            failures.append(f"{row['path']}: forbidden {row['forbidden']!r}")
    for row in manifest.get("script_reads", []):
        text = (REPO_ROOT / row["path"]).read_text(encoding="utf-8")
        if row["required"] not in text:
            failures.append(f"{row['path']}: does not read {row['required']!r}")
    return failures


def verify_forbidden_active_references(rows: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    for row in rows:
        pattern = row["pattern"]
        for search_path in row["search_paths"]:
            path = REPO_ROOT / search_path
            if path.is_file():
                files = [path]
            elif path.is_dir():
                files = [item for item in path.rglob("*") if item.is_file()]
            else:
                continue
            for file_path in files:
                if pattern in file_path.read_text(encoding="utf-8", errors="ignore"):
                    failures.append(f"{file_path.relative_to(REPO_ROOT)}: forbidden active reference {pattern}")
    return failures


if __name__ == "__main__":
    sys.exit(main())
