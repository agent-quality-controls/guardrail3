#!/usr/bin/env python3
from __future__ import annotations

import sys
import json
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST_PATH = (
    REPO_ROOT
    / ".plans"
    / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
)
PRE_COMMIT_HOOK_PATH = REPO_ROOT / ".githooks" / "pre-commit"


def load_toml(path: Path) -> dict:
    with path.open("rb") as file:
        return tomllib.load(file)


def manifest_path_from_argv(argv: list[str]) -> Path:
    if not argv:
        return DEFAULT_MANIFEST_PATH
    if len(argv) == 2 and argv[0] == "--manifest":
        return (REPO_ROOT / argv[1]).resolve() if not Path(argv[1]).is_absolute() else Path(argv[1])
    raise SystemExit("usage: verify-fixtures.py [--manifest <path>]")


def relative_files(root: Path) -> list[str]:
    files: list[str] = []
    for path in root.rglob("*"):
        if path.is_file() or path.is_symlink():
            files.append(path.relative_to(root).as_posix())
    return sorted(files)


VALID_LEVELS = {
    "workspace_root_not_found",
    "workspace_root_found_guardrail_config_missing",
    "workspace_root_found_guardrail_config_invalid",
    "guardrail_config_valid_required_inputs_missing",
    "required_inputs_present_invalid",
    "required_inputs_valid_source_filetree_inputs_invalid",
    "required_inputs_valid_delegated_tools_missing",
    "delegated_tools_present_policy_invalid",
    "delegated_policy_valid_project_policy_violated",
    "project_policy_valid_clean",
}

VALID_STATES = {
    "workspace_root_found",
    "guardrail_config_present",
    "guardrail_config_valid",
    "required_inputs_present",
    "required_inputs_valid",
    "delegated_tools_present",
    "delegated_policy_valid",
    "project_policy_valid",
}

VALID_INVALID_STATES = {
    "activation_file_conflict",
    "workspace_root_not_found",
    "guardrail_config_missing",
    "guardrail_config_invalid",
    "required_inputs_missing",
    "required_inputs_invalid",
    "source_inputs_invalid",
    "filetree_inputs_invalid",
    "delegated_tools_missing",
    "delegated_policy_invalid",
    "project_policy_violated",
    "workspace_local_file_misplaced",
    "workspace_topology_invalid",
}

VALID_EXPECTED_EXITS = {"zero", "nonzero"}
VALID_FIXTURE_KINDS = {"clean"}

REPO_LEVELS = {
    "repo_root_invalid",
    "repo_root_crawlable_no_adoption",
    "repo_hooks_reachable_no_root_cargo",
    "repo_marker_pair_policy",
    "repo_root_adoption_pair_complete",
    "repo_default_root",
}

REPO_VALID_STATES = {
    "repo_root_found",
    "repo_root_directory",
    "repo_root_crawlable",
    "repo_markers_absent",
    "repo_marker_pair_complete",
    "repo_marker_pair_incomplete_visible",
    "repo_marker_pair_inverse_incomplete_visible",
    "repo_marker_pair_ignored_under_behavior_fixtures",
    "repo_topology_branch_reachable",
    "repo_hooks_branch_reachable",
    "repo_default_root_resolved",
}

REPO_INVALID_STATES = {
    "repo_root_missing",
    "repo_root_file",
    "repo_marker_pair_incomplete",
    "repo_hooks_missing",
    "repo_hook_steps_weakened",
    "repo_modular_hook_dispatch_invalid",
    "repo_modular_hook_script_not_executable",
    "repo_topology_nested_workspace",
}


def verify_fixture_metadata(
    fixture_id: str,
    metadata: dict,
    valid_levels: set[str],
    valid_states: set[str],
    valid_invalid_states: set[str],
) -> list[str]:
    failures: list[str] = []
    commands = metadata.get("commands")
    if metadata.get("expected_exit") not in VALID_EXPECTED_EXITS:
        failures.append(f"{fixture_id}: expected_exit must be one of {sorted(VALID_EXPECTED_EXITS)}")
    if metadata.get("run_from") != "repo":
        failures.append(f"{fixture_id}: run_from must be `repo`")
    if metadata.get("level") not in valid_levels:
        failures.append(f"{fixture_id}: invalid level {metadata.get('level')}")
    if not isinstance(commands, list) or not commands:
        failures.append(f"{fixture_id}: commands must be a non-empty list")
    else:
        for command in commands:
            if not isinstance(command, list) or not all(isinstance(part, str) for part in command):
                failures.append(f"{fixture_id}: command must be a list of strings: {command}")
    for key, valid_values in (
        ("valid_state", valid_states),
        ("intentionally_invalid", valid_invalid_states),
    ):
        values = metadata.get(key)
        if not isinstance(values, list):
            failures.append(f"{fixture_id}: {key} must be a list")
            continue
        unknown = sorted(value for value in values if value not in valid_values)
        if unknown:
            failures.append(f"{fixture_id}: {key} has unknown values: {unknown}")
    return failures


def main() -> int:
    manifest = load_toml(manifest_path_from_argv(sys.argv[1:]))
    fixture_set = manifest["fixture_set"]
    fixture_root = REPO_ROOT / fixture_set["root"]
    golden_records = golden_records_by_fixture(fixture_set.get("golden_output"))
    failures: list[str] = []
    is_repo_fixture_set = fixture_set["root"] == "behavior/fixtures/g3rs-validate-repo"
    valid_levels = REPO_LEVELS if is_repo_fixture_set else VALID_LEVELS
    valid_states = REPO_VALID_STATES if is_repo_fixture_set else VALID_STATES
    valid_invalid_states = REPO_INVALID_STATES if is_repo_fixture_set else VALID_INVALID_STATES

    if not fixture_root.is_dir():
        failures.append(f"missing fixture root: {fixture_set['root']}")

    if not PRE_COMMIT_HOOK_PATH.is_file():
        failures.append("missing pre-commit hook")
    else:
        hook_lines = shell_assignment_lines(PRE_COMMIT_HOOK_PATH.read_text(encoding="utf-8"))
        required_hook_assignments = [
            ("VALIDATION_STAGED_FILES", 'grep -vE \'^behavior/fixtures/\''),
            ("MIGRATION_FILES", '$(echo "$VALIDATION_STAGED_FILES"'),
            ("STAGED_PACKAGE_JSON", '$(echo "$VALIDATION_STAGED_FILES"'),
            ("RUST_RELEVANT_FILES", '$(echo "$VALIDATION_STAGED_FILES"'),
            ("TS_RELEVANT_FILES", '$(echo "$VALIDATION_STAGED_FILES"'),
        ]
        for variable_name, required_fragment in required_hook_assignments:
            if not assignment_contains(hook_lines, variable_name, required_fragment):
                failures.append(
                    "pre-commit hook does not exclude behavior fixtures from validation routing: "
                    + variable_name
                )

    for link in fixture_set.get("required_shared_links", []):
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
        if entry.get("baseline_required"):
            expected_count = len(metadata.get("commands", [])) if isinstance(metadata.get("commands"), list) else 0
            actual_count = golden_records.get(fixture_id, 0)
            if actual_count != expected_count:
                failures.append(
                    f"{fixture_id}: approved golden record count mismatch: expected {expected_count}, got {actual_count}"
                )
        for key in ("id", "tool", "expected_exit"):
            if key not in metadata:
                failures.append(f"{fixture_id}: missing metadata key {key}")
        failures.extend(
            verify_fixture_metadata(fixture_id, metadata, valid_levels, valid_states, valid_invalid_states)
        )
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
        failures.extend(verify_fixture_kind(fixture_id, entry, fixture_dir))
        path_prepend = metadata.get("path_prepend", [])
        if path_prepend:
            if not isinstance(path_prepend, list) or not all(isinstance(item, str) for item in path_prepend):
                failures.append(f"{fixture_id}: path_prepend must be a list of strings")
            else:
                for rel_path in path_prepend:
                    if not (fixture_dir / "repo" / rel_path).is_dir():
                        failures.append(f"{fixture_id}: path_prepend directory missing {rel_path}")

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
                if path_present(fixture_dir / forbidden):
                    failures.append(f"{fixture_id}: forbidden path exists {forbidden}")

    if failures:
        print("behavior-fixtures: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"behavior-fixtures: PASS fixtures:{len(expected_ids)}")
    return 0


def golden_records_by_fixture(raw_path: object) -> dict[str, int]:
    if not isinstance(raw_path, str):
        return {}
    path = REPO_ROOT / raw_path
    if not path.is_file():
        return {}
    data = json.loads(path.read_text(encoding="utf-8"))
    counts: dict[str, int] = {}
    for record in data.get("records", []):
        fixture_id = record.get("fixture_id")
        if isinstance(fixture_id, str):
            counts[fixture_id] = counts.get(fixture_id, 0) + 1
    return counts


def verify_fixture_kind(fixture_id: str, entry: dict, fixture_dir: Path) -> list[str]:
    fixture_kind = entry.get("fixture_kind")
    if fixture_id == "L80-project-policy-valid-clean" and fixture_kind != "clean":
        return [f"{fixture_id}: fixture_kind must be clean"]
    if fixture_kind is None:
        return []
    failures: list[str] = []
    if fixture_kind not in VALID_FIXTURE_KINDS:
        failures.append(f"{fixture_id}: invalid fixture_kind {fixture_kind}")
        return failures
    if fixture_kind == "clean":
        if entry.get("expected_exit") != "zero":
            failures.append(f"{fixture_id}: clean fixture must have expected_exit zero")
        if entry.get("baseline_required") is not True:
            failures.append(f"{fixture_id}: clean fixture must require baseline")
        if entry.get("closed_file_list") is not False:
            failures.append(f"{fixture_id}: clean fixture must keep closed_file_list false")
        required_files = {
            "fixture.toml",
            "repo/Cargo.toml",
            "repo/guardrail3-rs.toml",
            "repo/Cargo.lock",
        }
        configured_required = set(entry.get("required_files", []))
        missing_required = sorted(required_files - configured_required)
        for required in missing_required:
            failures.append(f"{fixture_id}: clean fixture missing required_files entry {required}")
        if "repo/target" not in entry.get("forbidden_paths", []):
            failures.append(f"{fixture_id}: clean fixture must forbid repo/target")
        for required in required_files:
            if not (fixture_dir / required).is_file():
                failures.append(f"{fixture_id}: clean fixture required file missing on disk {required}")
        target_dir = fixture_dir / "repo" / "target"
        if path_present(target_dir):
            failures.append(f"{fixture_id}: clean fixture has forbidden repo/target")
    return failures


def path_present(path: Path) -> bool:
    return path.exists() or path.is_symlink()


def shell_assignment_lines(hook: str) -> list[str]:
    return [
        line.strip()
        for line in hook.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


def assignment_contains(lines: list[str], variable_name: str, required_fragment: str) -> bool:
    prefix = f"{variable_name}="
    return any(line.startswith(prefix) and required_fragment in line for line in lines)


if __name__ == "__main__":
    sys.exit(main())
