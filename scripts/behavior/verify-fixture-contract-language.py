#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFESTS = [
    REPO_ROOT / ".plans" / "2026-05-15-145757-fixture-contract-and-replay-audit.md.manifest.toml",
    REPO_ROOT / ".plans" / "2026-05-15-151150-serde-first-fixture-output-migration.md.manifest.toml",
]
KEPT_MANIFEST = REPO_ROOT / ".plans" / "2026-05-15-143306-g3rs-kept-test-disposition-audit.md.manifest.toml"


def main() -> int:
    failures = verify()
    if failures:
        print("fixture-contract-language: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("fixture-contract-language: PASS")
    return 0


def verify() -> list[str]:
    failures: list[str] = []
    manifests = [load_toml(path) for path in MANIFESTS]

    for manifest in manifests:
        for row in manifest.get("required_phrase", []):
            if not isinstance(row, dict):
                failures.append("required_phrase row must be a table")
                continue
            path = row.get("path")
            text = row.get("text")
            if not isinstance(path, str) or not isinstance(text, str):
                failures.append("required_phrase row must define path and text")
                continue
            content = repo_path(path).read_text(encoding="utf-8")
            if text not in content:
                failures.append(f"{path}: missing required phrase {text!r}")

        forbidden = [
            str(row["text"])
            for row in manifest.get("forbidden_phrase", [])
            if isinstance(row, dict) and isinstance(row.get("text"), str)
        ]
        for row in manifest.get("scan_target", []):
            if not isinstance(row, dict) or not isinstance(row.get("path"), str):
                failures.append("scan_target row must define path")
                continue
            path = str(row["path"])
            content = repo_path(path).read_text(encoding="utf-8")
            for phrase in forbidden:
                if phrase in content:
                    failures.append(f"{path}: forbidden fixture-contract phrase {phrase!r}")

    kept_manifest = load_toml(KEPT_MANIFEST)
    actual_dispositions = {
        str(row["name"]): int(row["expected_rows"])
        for row in kept_manifest.get("disposition", [])
        if isinstance(row, dict) and isinstance(row.get("name"), str)
    }
    for manifest in manifests:
        expected_dispositions = disposition_counts(manifest)
        if not expected_dispositions:
            continue
        if actual_dispositions != expected_dispositions:
            failures.append(
                "kept disposition manifest drifted from fixture contract manifest: "
                f"expected {sorted(expected_dispositions.items())}, got {sorted(actual_dispositions.items())}"
            )

    return failures


def disposition_counts(manifest: dict[str, Any]) -> dict[str, int]:
    rows = manifest.get("disposition", [])
    if not rows:
        rows = manifest.get("approved_disposition", [])
    return {
        str(row["name"]): int(row["expected_rows"])
        for row in rows
        if isinstance(row, dict) and isinstance(row.get("name"), str)
    }


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def repo_path(path: str) -> Path:
    return REPO_ROOT / path


if __name__ == "__main__":
    sys.exit(main())
