#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = REPO_ROOT / ".plans" / "2026-05-18-094322-runtime-contract-guardrails.md.manifest.toml"
RULE_ID_RE = re.compile(r'const ID: &str = "([^"]+)";')


def main() -> int:
    manifest = load_toml(MANIFEST_PATH)
    failures: list[str] = []
    verify_rules(manifest, failures)
    verify_parser_contracts(manifest, failures)
    verify_fixture_contracts(manifest, failures)
    verify_hook_contracts(manifest, failures)
    verify_dependency_hygiene(failures)
    if failures:
        print("runtime-contract-guardrails: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("runtime-contract-guardrails: PASS")
    return 0


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_rules(manifest: dict[str, Any], failures: list[str]) -> None:
    for row in manifest.get("rule", []):
        if not isinstance(row, dict):
            failures.append("rule row must be a table")
            continue
        rule_id = string_field(row, "id", failures)
        owner = repo_path(string_field(row, "owner", failures), failures)
        if not owner:
            continue
        if not owner.exists():
            failures.append(f"{owner.relative_to(REPO_ROOT)}: rule owner missing for {rule_id}")
            continue
        active_ids = active_rule_ids(owner)
        if rule_id not in active_ids:
            failures.append(f"{owner.relative_to(REPO_ROOT)}: missing active rule ID {rule_id}")


def active_rule_ids(root: Path) -> set[str]:
    ids: set[str] = set()
    for path in root.rglob("*.rs"):
        if any(part in {"target", ".cargo-target"} for part in path.parts):
            continue
        ids.update(RULE_ID_RE.findall(path.read_text(encoding="utf-8", errors="replace")))
    return ids


def verify_parser_contracts(manifest: dict[str, Any], failures: list[str]) -> None:
    for row in manifest.get("parser_contract", []):
        if not isinstance(row, dict):
            failures.append("parser_contract row must be a table")
            continue
        owner = repo_path(string_field(row, "owner", failures), failures)
        parser = string_field(row, "parser", failures)
        fields = string_list_field(row, "fields", failures)
        if not owner:
            continue
        cargo_toml = owner / "crates" / "runtime" / "Cargo.toml"
        if not cargo_toml.exists():
            cargo_toml = owner / "Cargo.toml"
        if not cargo_toml.exists():
            failures.append(f"{owner.relative_to(REPO_ROOT)}: parser owner has no Cargo.toml")
            continue
        cargo_text = cargo_toml.read_text(encoding="utf-8", errors="replace")
        if parser not in cargo_text:
            failures.append(f"{cargo_toml.relative_to(REPO_ROOT)}: missing parser dependency {parser}")
        source_text = "\n".join(
            path.read_text(encoding="utf-8", errors="replace")
            for path in owner.rglob("*.rs")
            if "target" not in path.parts
        )
        for field in fields:
            field_parts = [rust_field_name(part) for part in field.split(".")]
            if not all(part in source_text for part in field_parts):
                failures.append(f"{owner.relative_to(REPO_ROOT)}: parser field {field} is not extracted")


def rust_field_name(value: str) -> str:
    output: list[str] = []
    for index, char in enumerate(value.replace("-", "_")):
        if char.isupper() and index > 0:
            output.append("_")
        output.append(char.lower())
    return "".join(output)


def verify_fixture_contracts(manifest: dict[str, Any], failures: list[str]) -> None:
    for row in manifest.get("fixture_contract", []):
        if not isinstance(row, dict):
            failures.append("fixture_contract row must be a table")
            continue
        suite = string_field(row, "suite", failures)
        family = string_field(row, "family", failures)
        clean_fixture = string_field(row, "clean_fixture", failures)
        broken_fixture = string_field(row, "broken_fixture", failures)
        required_rule = string_field(row, "required_rule", failures)
        verify_fixture_file(suite, family, clean_fixture, required_rule, failures)
        verify_fixture_file(suite, family, broken_fixture, required_rule, failures)
        verify_golden_output(suite, clean_fixture, required_rule, False, failures)
        verify_golden_output(suite, broken_fixture, required_rule, True, failures)


def verify_fixture_file(
    suite: str,
    family: str,
    fixture_id: str,
    required_rule: str,
    failures: list[str],
) -> None:
    path = REPO_ROOT / "behavior" / "fixtures" / suite / family / fixture_id / "fixture.toml"
    if not path.exists():
        failures.append(f"{path.relative_to(REPO_ROOT)}: fixture missing")
        return
    text = path.read_text(encoding="utf-8", errors="replace")
    if required_rule not in text and "R00-clean-golden" not in fixture_id:
        failures.append(f"{path.relative_to(REPO_ROOT)}: fixture does not target {required_rule}")


def verify_golden_output(
    suite: str,
    fixture_id: str,
    required_rule: str,
    must_break: bool,
    failures: list[str],
) -> None:
    path = REPO_ROOT / "behavior" / "golden" / suite / "approved.normalized.json"
    if not path.exists():
        failures.append(f"{path.relative_to(REPO_ROOT)}: approved output missing")
        return
    data = json.loads(path.read_text(encoding="utf-8"))
    records = {
        record.get("fixture_id"): record
        for record in data.get("records", [])
        if isinstance(record, dict)
    }
    record = records.get(fixture_id)
    if not isinstance(record, dict):
        failures.append(f"{path.relative_to(REPO_ROOT)}: fixture {fixture_id} missing from approved output")
        return
    stdout = record.get("stdout")
    if not isinstance(stdout, str):
        failures.append(f"{path.relative_to(REPO_ROOT)}: fixture {fixture_id} stdout is not a string")
        return
    if required_rule not in stdout:
        failures.append(f"{fixture_id}: approved output does not mention {required_rule}")
    if must_break and not re.search(rf"^\[(?:Error|Warn)\] {re.escape(required_rule)} ", stdout, re.MULTILINE):
        failures.append(f"{fixture_id}: approved output does not break {required_rule}")


def verify_hook_contracts(manifest: dict[str, Any], failures: list[str]) -> None:
    for row in manifest.get("hook_contract", []):
        if not isinstance(row, dict):
            failures.append("hook_contract row must be a table")
            continue
        required_command = string_field(row, "required_command", failures)
        critical_command = string_field(row, "critical_command", failures)
        contract_text = read_all(REPO_ROOT / "packages" / "rs" / "deps" / "g3rs-deps-hook-contract")
        hook_types_text = read_all(REPO_ROOT / "packages" / "rs" / "hooks" / "g3rs-hooks-contract-types")
        hook_checks_text = read_all(REPO_ROOT / "packages" / "rs" / "hooks")
        if required_command not in contract_text:
            failures.append(f"deps hook contract does not require {required_command}")
        if critical_command not in contract_text:
            failures.append(f"deps hook contract does not mark {critical_command} critical")
        if required_command not in hook_types_text:
            failures.append(f"hook contract types do not define {required_command}")
        if (
            "cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked"
            not in hook_types_text + hook_checks_text
        ):
            failures.append(
                "hook command matching does not expose cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked"
            )


def verify_dependency_hygiene(failures: list[str]) -> None:
    changed_roots = [
        REPO_ROOT / "packages" / "ts" / "package" / "g3ts-package-ingestion",
        REPO_ROOT / "packages" / "rs" / "cargo" / "g3rs-cargo-ingestion",
    ]
    for root in changed_roots:
        text = "\n".join(
            path.read_text(encoding="utf-8", errors="replace")
            for path in root.rglob("Cargo.toml")
            if "target" not in path.parts
        )
        if "serde_yaml" in text:
            failures.append(f"{root.relative_to(REPO_ROOT)}: deprecated serde_yaml must not be used")


def read_all(root: Path) -> str:
    chunks: list[str] = []
    for path in root.rglob("*"):
        if not path.is_file() or any(part in {"target", ".cargo-target"} for part in path.parts):
            continue
        if path.suffix not in {".rs", ".toml"}:
            continue
        chunks.append(path.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(chunks)


def repo_path(value: str, failures: list[str]) -> Path | None:
    if not value:
        return None
    path = REPO_ROOT / value
    if REPO_ROOT not in path.resolve().parents and path.resolve() != REPO_ROOT:
        failures.append(f"{value}: path escapes repo root")
        return None
    return path


def string_field(row: dict[str, Any], name: str, failures: list[str]) -> str:
    value = row.get(name)
    if isinstance(value, str) and value:
        return value
    failures.append(f"{name} must be a non-empty string")
    return ""


def string_list_field(row: dict[str, Any], name: str, failures: list[str]) -> list[str]:
    value = row.get(name)
    if isinstance(value, list) and all(isinstance(item, str) and item for item in value):
        return list(value)
    failures.append(f"{name} must be a list of non-empty strings")
    return []


if __name__ == "__main__":
    sys.exit(main())
