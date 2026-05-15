#!/usr/bin/env python3
from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
from functools import lru_cache
from pathlib import Path
from typing import Any

from replay_common import (
    REPO_ROOT,
    command_cwd,
    copy_shared_fixture_inputs,
    fixture_copy_ignore,
    fixture_hash,
    load_fixture_metadata,
    normalize_output,
    prepare_runtime_fixture,
)


SCHEMA_VERSION = "g3rs-code-ingestion-replay-v1"
INGESTION_MANIFEST_PATH = REPO_ROOT / "packages" / "rs" / "code" / "g3rs-code-ingestion" / "Cargo.toml"
BINARY_NAME = "g3rs-code-ingestion-fixture-output"


def main() -> int:
    fixture_metadata_paths = parse_args(sys.argv[1:])
    records = []

    for fixture_metadata_path in fixture_metadata_paths:
        fixture_dir = fixture_metadata_path.parent
        fixture_id = fixture_dir.name
        metadata = load_fixture_metadata(fixture_dir)
        records.append(replay_ingestion_record(fixture_id, fixture_dir, metadata))

    output = {
        "schema_version": SCHEMA_VERSION,
        "tool": BINARY_NAME,
        "records": records,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0


def parse_args(argv: list[str]) -> list[Path]:
    if not argv:
        raise SystemExit("usage: fixture3-g3rs-code-ingestion.py <fixture.toml>...")
    fixture_paths = [absolute_path(path) for path in argv]
    for fixture_path in fixture_paths:
        if fixture_path.name != "fixture.toml":
            raise SystemExit(f"fixture path must end with fixture.toml: {fixture_path}")
    return fixture_paths


def absolute_path(raw_path: str) -> Path:
    path = Path(raw_path)
    return path if path.is_absolute() else REPO_ROOT / path


def replay_ingestion_record(
    fixture_id: str,
    fixture_dir: Path,
    metadata: dict[str, Any],
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="g3rs-code-ingestion-") as temp_root:
        temp_root_path = Path(temp_root)
        copy_shared_fixture_inputs(temp_root_path)
        runtime_fixture_dir = temp_root_path / fixture_dir.parent.name / fixture_dir.name
        runtime_fixture_dir.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(fixture_dir, runtime_fixture_dir, symlinks=False, ignore=fixture_copy_ignore)
        prepare_runtime_fixture(runtime_fixture_dir, metadata)
        workspace_path = command_cwd(runtime_fixture_dir, metadata)
        result = subprocess.run(
            [
                code_ingestion_fixture_binary(),
                "--path",
                str(workspace_path),
            ],
            cwd=REPO_ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
        stderr = normalize_output(result.stderr, runtime_fixture_dir)
        payload = (
            normalize_json_value(json.loads(result.stdout), runtime_fixture_dir)
            if result.returncode == 0
            else None
        )
        cwd = workspace_path.relative_to(runtime_fixture_dir).as_posix()

    return {
        "fixture_id": fixture_id,
        "fixture_hash": fixture_hash(fixture_dir),
        "command": [BINARY_NAME, "--path", "$REPO"],
        "cwd": cwd,
        "exit_code": result.returncode,
        "payload": payload,
        "stderr": stderr,
    }


@lru_cache(maxsize=1)
def code_ingestion_fixture_binary() -> str:
    subprocess.run(
        [
            "cargo",
            "build",
            "--quiet",
            "--manifest-path",
            str(INGESTION_MANIFEST_PATH),
            "-p",
            BINARY_NAME,
            "--bin",
            BINARY_NAME,
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
            str(INGESTION_MANIFEST_PATH),
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
    return str(Path(target_directory) / "debug" / BINARY_NAME)


def normalize_json_value(value: Any, fixture_dir: Path) -> Any:
    if isinstance(value, dict):
        return {key: normalize_json_value(nested, fixture_dir) for key, nested in value.items()}
    if isinstance(value, list):
        return [normalize_json_value(nested, fixture_dir) for nested in value]
    if isinstance(value, str):
        return normalize_json_string(value, fixture_dir)
    return value


def normalize_json_string(value: str, fixture_dir: Path) -> str:
    replacements = [
        (fixture_dir / "repo", "$REPO"),
        (fixture_dir, "$FIXTURE"),
        (fixture_dir.parent.parent, "$FIXTURE_ROOT"),
        (REPO_ROOT / ".cargo-target", "$TARGET"),
    ]
    normalized = value
    for path, marker in replacements:
        normalized = normalized.replace(path.as_posix(), marker)
        normalized = normalized.replace(str(path), marker)
    return normalized.replace("/private$REPO", "$REPO")


if __name__ == "__main__":
    sys.exit(main())
