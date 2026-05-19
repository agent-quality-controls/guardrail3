#!/usr/bin/env python3
from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST = REPO_ROOT / ".plans/2026-05-19-183426-g3ts-setup-flow-hardening.md.manifest.toml"
G3TS_MANIFEST = REPO_ROOT / "apps/guardrail3-ts/Cargo.toml"
G3TS_BIN = REPO_ROOT / "apps/guardrail3-ts/target/debug/g3ts"


def main() -> int:
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    failures: list[str] = []
    build_g3ts(failures)
    verify_cli_help(manifest, failures)
    verify_repo_validation(manifest, failures)
    verify_workspace_validation(manifest, failures)
    verify_init_workspace(manifest, failures)
    verify_toolchain_inventory(manifest, failures)
    verify_source(manifest, failures)
    if failures:
        print("FAIL g3ts setup flow hardening manifest")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("PASS g3ts setup flow hardening manifest")
    return 0


def build_g3ts(failures: list[str]) -> None:
    result = subprocess.run(
        [
            "cargo",
            "build",
            "--quiet",
            "--manifest-path",
            str(G3TS_MANIFEST),
            "-p",
            "g3ts",
            "--bin",
            "g3ts",
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        failures.append(f"cargo build failed: {result.stderr[-2000:]}")


def verify_cli_help(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("cli_help_contains", []):
        result = run_g3ts(row["command"])
        output = result.stdout + result.stderr
        if result.returncode != 0:
            failures.append(f"g3ts {' '.join(row['command'])} exited {result.returncode}")
            continue
        if row["text"] not in output:
            failures.append(f"g3ts {' '.join(row['command'])} missing {row['text']}")


def verify_repo_validation(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("repo_validation", []):
        with temp_repo() as repo:
            write_files(repo, row["files"])
            if row.get("init_repo", False):
                init = run_g3ts(["init", "repo", "--path", str(repo)])
                if init.returncode != 0:
                    failures.append(f"{row['case']}: init repo failed: {(init.stdout + init.stderr)[-1000:]}")
                    continue
            result = run_g3ts(expand_command(row["command"], repo))
            output = result.stdout + result.stderr
            expected_exit = row.get("expect_exit", 0)
            if result.returncode != expected_exit:
                failures.append(
                    f"{row['case']}: expected exit {expected_exit}, got {result.returncode}: {output[-1000:]}"
                )
            for expected in row["expect_stdout_contains"]:
                if expected not in output:
                    failures.append(f"{row['case']}: output missing {expected}")


def verify_workspace_validation(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("workspace_validation", []):
        with temp_repo() as repo:
            write_files(repo, row["files"])
            result = run_g3ts(expand_command(row["command"], repo))
            output = result.stdout + result.stderr
            expected_exit = row.get("expect_exit", 0)
            if result.returncode != expected_exit:
                failures.append(
                    f"{row['case']}: expected exit {expected_exit}, got {result.returncode}: {output[-1000:]}"
                )
            for forbidden in row.get("expect_stdout_not_contains", []):
                if forbidden in output:
                    failures.append(f"{row['case']}: output contained forbidden text {forbidden}")


def verify_init_workspace(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("init_workspace", []):
        with temp_repo() as repo:
            write_files(repo, row["files"])
            result = run_g3ts(expand_command(row["command"], repo))
            output = result.stdout + result.stderr
            if result.returncode != 0:
                failures.append(f"{row['case']}: init exited {result.returncode}: {output[-1000:]}")
                continue
            for rel_path in row["expect_files"]:
                if not (repo / rel_path).is_file():
                    failures.append(f"{row['case']}: missing file {rel_path}")
            package_json = (repo / "package.json").read_text(encoding="utf-8")
            for expected in row["expect_package_json_contains"]:
                if expected not in package_json:
                    failures.append(f"{row['case']}: package.json missing {expected}")


def verify_toolchain_inventory(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("toolchain_inventory", []):
        with temp_repo() as repo:
            write_files(repo, ["package.json"])
            init = run_g3ts(["init", "workspace", "--path", str(repo)])
            if init.returncode != 0:
                failures.append(f"{row['case']}: init failed: {(init.stdout + init.stderr)[-1000:]}")
                continue
            result = run_g3ts(expand_command(row["command"], repo))
            output = result.stdout + result.stderr
            for expected in row["expect_stdout_contains"]:
                if expected not in output:
                    failures.append(f"{row['case']}: output missing {expected}")


def verify_source(manifest: dict[str, object], failures: list[str]) -> None:
    for row in manifest.get("source_contains", []):
        text = (REPO_ROOT / row["path"]).read_text(encoding="utf-8")
        if row["text"] not in text:
            failures.append(f"{row['path']} missing {row['text']}")


def write_files(repo: Path, files: list[str]) -> None:
    for rel_path in files:
        path = repo / rel_path
        path.parent.mkdir(parents=True, exist_ok=True)
        if rel_path == "package.json":
            path.write_text(
                json.dumps({"name": "fixture", "version": "0.0.0"}, indent=2) + "\n",
                encoding="utf-8",
            )
        elif rel_path == "guardrail3-ts.toml":
            path.write_text("profile = \"typescript\"\n", encoding="utf-8")
        else:
            path.write_text("", encoding="utf-8")


def expand_command(command: list[str], repo: Path) -> list[str]:
    return [str(repo) if token == "{repo}" else token for token in command]


def run_g3ts(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(G3TS_BIN), *argv],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


class temp_repo:
    def __enter__(self) -> Path:
        self.path = Path(tempfile.mkdtemp(prefix="g3ts-setup-flow-"))
        subprocess.run(["git", "init", "--quiet"], cwd=self.path, check=True)
        return self.path

    def __exit__(self, *args: object) -> None:
        shutil.rmtree(self.path)


if __name__ == "__main__":
    sys.exit(main())
