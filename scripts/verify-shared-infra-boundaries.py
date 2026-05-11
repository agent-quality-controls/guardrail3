#!/usr/bin/env python3
"""Verify shared infrastructure package boundaries from the manifest."""

from __future__ import annotations

import fnmatch
import subprocess
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFESTS = [
    ROOT / ".plans/2026-05-11-105304-clean-shared-infra-boundaries.manifest.toml",
    ROOT / ".plans/2026-05-11-114847-rename-g3rs-g3ts-toml-parsers.manifest.toml",
]
EXCLUDED_PARTS = {
    ".baselines",
    ".git",
    ".plans",
    ".worklogs",
    "legacy",
    "node_modules",
    "target",
}


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def load_manifest() -> dict[str, list[dict[str, object]]]:
    merged: dict[str, list[dict[str, object]]] = {}
    for manifest in MANIFESTS:
        with manifest.open("rb") as handle:
            parsed = tomllib.load(handle)
        for key, value in parsed.items():
            if isinstance(value, list):
                merged.setdefault(key, []).extend(value)
    return merged


def active_files() -> list[Path]:
    result = subprocess.run(
        ["git", "ls-files", "-co", "--exclude-standard"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr)
    files: list[Path] = []
    for line in result.stdout.splitlines():
        path = ROOT / line
        if EXCLUDED_PARTS.intersection(path.relative_to(ROOT).parts):
            continue
        if path.is_file():
            files.append(path)
    return files


ACTIVE_FILES = active_files()


def files_matching(pattern: str) -> list[Path]:
    return sorted(path for path in ACTIVE_FILES if fnmatch.fnmatch(rel(path), pattern))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def fail(layer: str, failures: list[str]) -> bool:
    if not failures:
        print(f"{layer} status:PASS")
        return False
    print(f"{layer} status:FAIL")
    for failure in failures:
        print(f"  - {failure}")
    return True


def layer_tree(manifest: dict[str, list[dict[str, object]]]) -> bool:
    failures: list[str] = []
    for row in manifest.get("tree_present", []):
        path = ROOT / str(row["path"])
        if not path.exists():
            failures.append(f"missing required path: {rel(path)}")
    for row in manifest.get("tree_absent", []):
        path = ROOT / str(row["path"])
        if path.exists():
            failures.append(f"forbidden path still exists: {rel(path)}")
    return fail("layer:1-tree", failures)


def layer_cargo_deps(manifest: dict[str, list[dict[str, object]]]) -> bool:
    failures: list[str] = []
    for row in manifest.get("cargo_dependency", []):
        manifest_path = ROOT / str(row["manifest"])
        content = read(manifest_path)
        package = str(row["package"])
        dep_path = str(row["path"])
        if f"{package} = {{" not in content:
            failures.append(f"{rel(manifest_path)} missing dependency {package}")
        if f'path = "{dep_path}"' not in content:
            failures.append(f"{rel(manifest_path)} dependency {package} does not point at {dep_path}")

    for row in manifest.get("forbidden_cargo_text", []):
        text = str(row["text"])
        for path in files_matching(str(row["scope"])):
            if text in read(path):
                failures.append(f"{rel(path)} contains forbidden Cargo text `{text}`")

    for table_name in ("forbidden_ts_dependency", "forbidden_rs_dependency"):
        for row in manifest.get(table_name, []):
            package = str(row["package"])
            for path in files_matching(str(row["scope"])):
                if f"{package} =" in read(path):
                    failures.append(f"{rel(path)} depends on forbidden package `{package}`")

    return fail("layer:2-cargo-deps", failures)


def layer_source_imports(manifest: dict[str, list[dict[str, object]]]) -> bool:
    failures: list[str] = []
    for row in manifest.get("forbidden_source_text", []):
        text = str(row["text"])
        for path in files_matching(str(row["scope"])):
            if text in read(path):
                failures.append(f"{rel(path)} contains forbidden source text `{text}`")

    for row in manifest.get("forbidden_active_text", []):
        text = str(row["text"])
        for path in files_matching(str(row["scope"])):
            if text in read(path):
                failures.append(f"{rel(path)} contains forbidden active text `{text}`")

    for row in manifest.get("required_active_text", []):
        path = ROOT / str(row["file"])
        text = str(row["text"])
        if text not in read(path):
            failures.append(f"{rel(path)} missing required active text `{text}`")

    for row in manifest.get("ts_policy_reader", []):
        path = ROOT / str(row["file"])
        parser = str(row["parser"])
        content = read(path)
        if parser not in content:
            failures.append(f"{rel(path)} does not use required parser `{parser}`")
    return fail("layer:3-source-imports", failures)


def layer_parser_schema(manifest: dict[str, list[dict[str, object]]]) -> bool:
    failures: list[str] = []
    for row in manifest.get("parser_schema_forbidden", []):
        path = ROOT / str(row["file"])
        text = str(row["text"])
        if text in read(path):
            failures.append(f"{rel(path)} contains forbidden schema text `{text}`")
    for row in manifest.get("parser_schema_required", []):
        path = ROOT / str(row["file"])
        text = str(row["text"])
        if text not in read(path):
            failures.append(f"{rel(path)} missing required schema text `{text}`")
    return fail("layer:4-parser-schema", failures)


def layer_validate(manifest: dict[str, list[dict[str, object]]]) -> bool:
    failures: list[str] = []
    for row in manifest.get("verify_command", []):
        cmd = str(row["cmd"])
        if cmd == "python3 scripts/verify-shared-infra-boundaries.py":
            continue
        result = subprocess.run(
            cmd,
            cwd=ROOT,
            shell=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )
        if result.returncode != 0:
            failures.append(f"`{cmd}` exited {result.returncode}\n{result.stdout}")
    return fail("layer:5-validate", failures)


def main() -> int:
    manifest = load_manifest()
    failed = [
        layer_tree(manifest),
        layer_cargo_deps(manifest),
        layer_source_imports(manifest),
        layer_parser_schema(manifest),
        layer_validate(manifest),
    ]
    if any(failed):
        print("verify-shared-infra-boundaries: FAIL")
        return 1
    print("verify-shared-infra-boundaries: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
