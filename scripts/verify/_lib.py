#!/usr/bin/env python3
"""Shared utilities for manifest verifier scripts.

Loads the manifest TOML and provides typed accessors for each section.
Verifier scripts import this module via a small runner shell wrapper.
"""

from __future__ import annotations

import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = (
    REPO_ROOT
    / ".plans"
    / "2026-05-06-215807-fix-rust-verifier-workspace-routing-regression.manifest.toml"
)


def load_manifest() -> dict:
    with MANIFEST_PATH.open("rb") as fp:
        return tomllib.load(fp)


def section(manifest: dict, name: str) -> list:
    return manifest.get(name, [])


def emit(layer: str, status: str, detail: str = "") -> int:
    """Print a result line and return 0 for PASS, 1 for FAIL."""
    line = f"layer:{layer} status:{status}"
    if detail:
        line += f" detail:{detail}"
    print(line)
    return 0 if status == "PASS" else 1


def emit_diff(layer: str, missing: list, unexpected: list) -> int:
    if not missing and not unexpected:
        return emit(layer, "PASS")
    print(f"layer:{layer} status:FAIL")
    for item in missing:
        print(f"  MISSING: {item}")
    for item in unexpected:
        print(f"  UNEXPECTED: {item}")
    return 1
