#!/usr/bin/env python3
from __future__ import annotations

import json
import hashlib
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
        (REPO_ROOT / ".cargo-target", "$TARGET"),
    ]
    normalized = text
    for path, marker in replacements:
        normalized = normalized.replace(path.as_posix(), marker)
        normalized = normalized.replace(str(path), marker)
    return normalized.replace("\\", "/")


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
    return subprocess.run(
        [tool_executable(tool), *argv],
        cwd=command_cwd(fixture_dir, metadata),
        text=True,
        capture_output=True,
        check=False,
    )


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
        runtime_fixture_dir = Path(temp_root) / fixture_dir.name
        shutil.copytree(fixture_dir, runtime_fixture_dir, symlinks=True)
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


def baseline_path(baseline_root: Path, fixture_id: str, command_index: int) -> Path:
    return baseline_root / fixture_id / f"command-{command_index:02}.json"


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
