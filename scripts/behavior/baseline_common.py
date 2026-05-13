#!/usr/bin/env python3
from __future__ import annotations

import json
import hashlib
import os
import re
import shutil
import subprocess
import tempfile
from functools import lru_cache
from pathlib import Path
from typing import Any

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
RUNNER_VERSION = "1"
NORMALIZER_VERSION = "1"
OUTPUT_SCHEMA_VERSION = "1"
G3RS_MANIFEST_PATH = REPO_ROOT / "apps" / "guardrail3-rs" / "Cargo.toml"
DELEGATED_TOOL_NAMES = {
    "cargo-deny",
    "cargo-dupes",
    "cargo-machete",
    "cargo-mutants",
    "g3rs",
    "gitleaks",
}


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def manifest_path_from_argv(argv: list[str]) -> Path:
    if not argv:
        return DEFAULT_MANIFEST_PATH
    if len(argv) == 2 and argv[0] == "--manifest":
        return (REPO_ROOT / argv[1]).resolve() if not Path(argv[1]).is_absolute() else Path(argv[1])
    raise SystemExit("usage: script [--manifest <path>]")


def load_manifest(path: Path | None = None) -> dict[str, Any]:
    return load_toml(path or DEFAULT_MANIFEST_PATH)


def load_fixture_metadata(fixture_dir: Path) -> dict[str, Any]:
    return load_toml(fixture_dir / "fixture.toml")


def git_head() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=True,
    )
    return result.stdout.strip()


def command_cwd(fixture_dir: Path, metadata: dict[str, Any]) -> Path:
    run_from = metadata.get("run_from")
    if run_from == "repo":
        return fixture_dir / "repo"
    raise ValueError(f"unsupported run_from: {run_from}")


@lru_cache(maxsize=1)
def g3rs_candidate_binary() -> Path:
    subprocess.run(
        [
            "cargo",
            "build",
            "--quiet",
            "--manifest-path",
            str(G3RS_MANIFEST_PATH),
            "-p",
            "guardrail3-rs",
            "--bin",
            "g3rs",
        ],
        cwd=REPO_ROOT,
        check=True,
    )
    metadata = subprocess.run(
        [
            "cargo",
            "metadata",
            "--quiet",
            "--manifest-path",
            str(G3RS_MANIFEST_PATH),
            "--no-deps",
            "--format-version",
            "1",
        ],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=True,
    )
    target_directory = json.loads(metadata.stdout)["target_directory"]
    return Path(target_directory) / "debug" / "g3rs"


def tool_executable(tool: str) -> Path | str:
    if tool == "g3rs":
        return g3rs_candidate_binary()
    return tool


def normalize_output(text: str, fixture_dir: Path) -> str:
    replacements = [
        (fixture_dir / "repo", "$REPO"),
        (fixture_dir, "$FIXTURE"),
        (fixture_dir.parent.parent, "$FIXTURE_ROOT"),
        (REPO_ROOT / ".cargo-target", "$TARGET"),
    ]
    normalized = text
    for path, marker in replacements:
        normalized = normalized.replace(path.as_posix(), marker)
        normalized = normalized.replace(str(path), marker)
    normalized = normalized.replace("/private$REPO", "$REPO")
    normalized = normalized.replace("\\", "/")
    normalized = re.sub(r"^    Blocking waiting for file lock on package cache\n", "", normalized, flags=re.MULTILINE)
    normalized = re.sub(r"target\(s\) in [0-9.]+s", "target(s) in $TIME", normalized)
    normalized = re.sub(r"finished in [0-9.]+s", "finished in $TIME", normalized)
    return re.sub(r"(\.cargo-target/debug/deps/[A-Za-z0-9_]+)-[0-9a-f]{16}", r"\1-$HASH", normalized)


def fixture_hash(fixture_dir: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(item for item in fixture_dir.rglob("*") if item.is_file() or item.is_symlink()):
        rel_path = path.relative_to(fixture_dir).as_posix()
        digest.update(rel_path.encode("utf-8"))
        digest.update(b"\0")
        if path.is_symlink():
            digest.update(b"symlink:")
            digest.update(path.readlink().as_posix().encode("utf-8"))
        else:
            digest.update(path.read_bytes())
        digest.update(b"\0")
    return f"sha256:{digest.hexdigest()}"


def run_command(tool: str, fixture_dir: Path, metadata: dict[str, Any], argv: list[str]) -> subprocess.CompletedProcess[str]:
    env = None
    path_prepend = metadata.get("path_prepend", [])
    if metadata.get("runner_mode") == "path_without_delegated_tools":
        env = os.environ.copy()
        env["PATH"] = rust_toolchain_path_without_delegated_tools()
        if metadata.get("blocked_cargo_subcommands"):
            blocker_path = cargo_subcommand_blocker_path(metadata["blocked_cargo_subcommands"])
            env["PATH"] = os.pathsep.join([blocker_path, env["PATH"]])
    elif path_prepend:
        env = os.environ.copy()
        prepend_paths = [
            str(command_cwd(fixture_dir, metadata) / rel_path)
            for rel_path in path_prepend
        ]
        env["PATH"] = os.pathsep.join([*prepend_paths, env.get("PATH", "")])
    return subprocess.run(
        [tool_executable(tool), *argv],
        cwd=command_cwd(fixture_dir, metadata),
        env=env,
        text=True,
        capture_output=True,
        check=False,
    )


def cargo_subcommand_blocker_path(blocked_subcommands: list[str]) -> str:
    real_cargo = shutil.which("cargo")
    if real_cargo is None:
        raise RuntimeError("cargo not found on PATH")
    temp_bin = Path(tempfile.mkdtemp(prefix="g3rs-behavior-cargo-blocker-"))
    cargo_wrapper = temp_bin / "cargo"
    cases = "\n".join(
        f'if [ "$1" = "{subcommand}" ]; then exit 101; fi'
        for subcommand in blocked_subcommands
    )
    cargo_wrapper.write_text(f'#!/bin/sh\n{cases}\nexec "{real_cargo}" "$@"\n')
    os.chmod(cargo_wrapper, 0o755)
    return temp_bin.as_posix()


@lru_cache(maxsize=1)
def rust_toolchain_path_without_delegated_tools() -> str:
    temp_bin = Path(tempfile.mkdtemp(prefix="g3rs-behavior-path-"))
    for dir_name in os.environ.get("PATH", "").split(os.pathsep):
        if not dir_name:
            continue
        source_dir = Path(dir_name)
        if not source_dir.is_dir():
            continue
        for source in source_dir.iterdir():
            if source.name in DELEGATED_TOOL_NAMES:
                continue
            try:
                is_runnable_file = source.is_file() and os.access(source, os.X_OK)
            except OSError:
                continue
            if not is_runnable_file:
                continue
            target = temp_bin / source.name
            if not target.exists():
                target.symlink_to(source)
    return temp_bin.as_posix()


def prepare_runtime_fixture(fixture_dir: Path, metadata: dict[str, Any]) -> None:
    if metadata.get("git_init") is not True:
        return
    repo_dir = command_cwd(fixture_dir, metadata)
    subprocess.run(["git", "init", "--quiet"], cwd=repo_dir, check=True)
    subprocess.run(["git", "config", "core.hooksPath", ".githooks"], cwd=repo_dir, check=True)


def output_record(
    tool: str,
    fixture_id: str,
    fixture_dir: Path,
    metadata: dict[str, Any],
    command_index: int,
    argv: list[str],
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="g3rs-behavior-") as temp_root:
        temp_root_path = Path(temp_root)
        copy_shared_fixture_inputs(temp_root_path)
        runtime_fixture_dir = temp_root_path / fixture_dir.parent.name / fixture_dir.name
        runtime_fixture_dir.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(fixture_dir, runtime_fixture_dir, symlinks=False, ignore=fixture_copy_ignore)
        prepare_runtime_fixture(runtime_fixture_dir, metadata)
        result = run_command(tool, runtime_fixture_dir, metadata, argv)
        cwd = command_cwd(runtime_fixture_dir, metadata).relative_to(runtime_fixture_dir).as_posix()
        stdout = normalize_output(result.stdout, runtime_fixture_dir)
        stderr = normalize_output(result.stderr, runtime_fixture_dir)
    return {
        "baseline_commit": git_head(),
        "fixture_id": fixture_id,
        "fixture_hash": fixture_hash(fixture_dir),
        "command_index": command_index,
        "command": [tool, *argv],
        "cwd": cwd,
        "normalizer_version": NORMALIZER_VERSION,
        "output_schema_version": OUTPUT_SCHEMA_VERSION,
        "runner_version": RUNNER_VERSION,
        "exit_code": result.returncode,
        "stdout": stdout,
        "stderr": stderr,
        "tool": tool,
    }


def copy_shared_fixture_inputs(temp_root: Path) -> None:
    shared_root = REPO_ROOT / "behavior" / "fixtures"
    for name in ("parsers", "shared"):
        source = shared_root / name
        if source.exists():
            shutil.copytree(source, temp_root / name, symlinks=False, ignore=fixture_copy_ignore)


def fixture_copy_ignore(_directory: str, names: list[str]) -> set[str]:
    return {name for name in names if name in {"target", ".git"}}


def baseline_path(baseline_root: Path, fixture_id: str, command_index: int) -> Path:
    return baseline_root / fixture_id / f"command-{command_index:02}.json"


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
