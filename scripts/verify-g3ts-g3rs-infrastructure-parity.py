#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path


REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / ".plans/2026-05-19-213005-g3ts-g3rs-infrastructure-parity.md.manifest.toml"
G3TS_BIN = REPO / "apps/guardrail3-ts/target/debug/g3ts"
G3RS_BIN = REPO / "apps/guardrail3-rs/target/debug/g3rs"


def main() -> int:
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    failures: list[str] = []

    verify_cli_surface(manifest, failures)
    verify_help(manifest, failures)
    verify_request_types(manifest, failures)
    verify_report_types(manifest, failures)
    verify_init_contracts(manifest, failures)
    verify_execution_ownership(manifest, failures)
    verify_selection_contracts(manifest, failures)
    verify_hook_contracts(manifest, failures)
    verify_fixture_roots(manifest, failures)
    verify_inventory_exit_contracts(manifest, failures)

    if failures:
        print("FAIL g3ts/g3rs infrastructure parity")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("PASS g3ts/g3rs infrastructure parity")
    return 0


def verify_cli_surface(manifest: dict[str, object], failures: list[str]) -> None:
    source_by_tool = {
        "g3ts": read("apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs"),
        "g3rs": read("apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs"),
    }
    for row in manifest.get("cli_surface", []):
        tool = row["tool"]
        text = source_by_tool[tool]
        for command in row["commands"]:
            first, second = command.split(" ", 1)
            if f"enum {first.title()}Command" not in text and first.title() not in text:
                failures.append(f"{tool} CLI source missing command group `{first}`")
            if second.title().replace(" ", "") not in text:
                failures.append(f"{tool} CLI source missing command `{command}`")
        for flag in row["workspace_flags"]:
            if flag_name(flag) not in text:
                failures.append(f"{tool} workspace CLI missing flag {flag}")
        for flag in row["repo_validate_flags"]:
            if flag_name(flag) not in text:
                failures.append(f"{tool} repo validate CLI missing flag {flag}")


def verify_help(manifest: dict[str, object], failures: list[str]) -> None:
    output_by_tool = {"g3ts": run_tool("g3ts", ["--help"]), "g3rs": run_tool("g3rs", ["--help"])}
    for row in manifest.get("help_section", []):
        tool = row["tool"]
        result = output_by_tool[tool]
        output = result.stdout + result.stderr
        if result.returncode != 0:
            failures.append(f"{tool} --help exited {result.returncode}: {output[-1000:]}")
            continue
        for section in row["required_sections"]:
            if section not in output:
                failures.append(f"{tool} --help missing section `{section}`")
        for forbidden in row["forbidden_text"]:
            if forbidden in output:
                failures.append(f"{tool} --help still contains forbidden text `{forbidden}`")


def verify_request_types(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("request_type", []):
        text = read(row["file"])
        if f"struct {row['type']}" not in text:
            failures.append(f"{row['file']} missing struct {row['type']}")
        for field in row["fields"]:
            if f"pub {field}" not in text:
                failures.append(f"{row['file']} missing field `{field}`")


def verify_report_types(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("report_type", []):
        text = read(row["file"])
        normalized = text.replace("std::path::", "")
        if f"struct {row['type']}" not in text:
            failures.append(f"{row['file']} missing struct {row['type']}")
        for field in row["fields"]:
            if f"pub {field}" not in normalized:
                failures.append(f"{row['file']} missing report field `{field}`")
        if f"fn {row['constructor']}" not in text:
            failures.append(f"{row['file']} missing constructor `{row['constructor']}`")


def verify_init_contracts(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("init_contract", []):
        text = read(row["file"])
        for expected in row["must_print"]:
            if expected not in text:
                failures.append(f"{row['tool']} init contract missing output `{expected}`")
        for forbidden in row["forbidden_calls"]:
            if forbidden in strip_init_function_names(text):
                failures.append(f"{row['tool']} init contract still calls `{forbidden}`")


def verify_execution_ownership(manifest: dict[str, object], failures: list[str]) -> None:
    required_fragments = {
        "workspace-adoption": ["workspace_adoption::check_repo"],
        "marker-pairs": ["marker_pairs::check_repo"],
        "tool-presence": ["tool_presence::check_required_tools_present"],
        "exit-code": ["highest_severity(&report, false)"],
    }
    for row in manifest.get("execution_ownership", []):
        validate_command = read(row["validate_command_file"])
        cli_runtime = read(row["cli_runtime_file"])
        for item in row["validate_command_must_own"]:
            for fragment in required_fragments[item]:
                if fragment not in validate_command:
                    failures.append(
                        f"{row['tool']} validate-command does not own `{item}` via `{fragment}`"
                    )
        for forbidden in row["cli_runtime_forbidden"]:
            if forbidden in cli_runtime:
                failures.append(f"{row['tool']} CLI runtime still owns `{forbidden}`")


def verify_selection_contracts(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("selection_contract", []):
        text = read(row["file"])
        for item in row["required_items"]:
            if item not in text:
                failures.append(f"{row['tool']} selection contract missing `{item}`")


def verify_hook_contracts(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("hook_contract", []):
        text = read(row["file"])
        for fragment in row["required_fragments"]:
            if fragment not in text:
                failures.append(f"{row['tool']} generated hook missing `{fragment}`")


def verify_fixture_roots(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("fixture_contract", []):
        root = REPO / row["root"]
        if not root.is_dir():
            failures.append(f"{row['tool']} fixture root missing: {row['root']}")
            continue
        fixture_files = list(root.rglob("fixture.toml"))
        if not fixture_files:
            failures.append(f"{row['tool']} fixture root has no fixture.toml files: {row['root']}")


def verify_inventory_exit_contracts(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("inventory_exit_contract", []):
        for command in row["commands"]:
            tool = command.split()[0]
            argv = command.split()[1:]
            result = run_tool(tool, argv)
            if result.returncode != row["must_exit"]:
                output = result.stdout + result.stderr
                failures.append(
                    f"{command} exited {result.returncode}, expected {row['must_exit']}: {output[-1000:]}"
                )


def run_tool(tool: str, argv: list[str]) -> subprocess.CompletedProcess[str]:
    binary = {"g3ts": G3TS_BIN, "g3rs": G3RS_BIN}[tool]
    return subprocess.run(
        [str(binary), *argv],
        cwd=REPO,
        capture_output=True,
        text=True,
        check=False,
    )


def read(path: str | Path) -> str:
    return (REPO / path).read_text(encoding="utf-8")


def flag_name(flag: str) -> str:
    return flag.removeprefix("--").replace("-", "_")


def strip_init_function_names(text: str) -> str:
    return text.replace("execute_init_repo", "").replace("execute_init_workspace", "")


if __name__ == "__main__":
    sys.exit(main())
