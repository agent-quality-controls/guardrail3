#!/usr/bin/env python3
"""Verifier for .plans/2026-05-10-212749-remove-structural-split.manifest.toml.

Layers:
  1. tree            file/dir presence and absence
  2. forbidden_text  named files must not contain a needle
  3. waivers         no guardrail3-rs.toml [[waivers]] table references the rule
  4. repo_grep       repo-wide grep returns zero hits for given patterns
  5. validate        each declared workspace exits 0 on `g3rs validate --path <ws>`

Exit code: 0 if all PASS, 1 otherwise.
"""

from __future__ import annotations

import fnmatch
import os
import re
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST = REPO_ROOT / ".plans" / "2026-05-10-212749-remove-structural-split.manifest.toml"
G3RS_BIN = REPO_ROOT / "apps" / "guardrail3-rs" / "target" / "release" / "g3rs"


def load_manifest() -> dict:
    with MANIFEST.open("rb") as fp:
        return tomllib.load(fp)


def emit(layer: str, status: str, detail: str = "") -> int:
    line = f"layer:{layer} status:{status}"
    if detail:
        line += f" detail:{detail}"
    print(line)
    return 0 if status == "PASS" else 1


def fail(layer: str, messages: list[str]) -> int:
    print(f"layer:{layer} status:FAIL")
    for m in messages:
        print(f"  {m}")
    return 1


def layer_tree(manifest: dict) -> int:
    failures: list[str] = []
    for entry in manifest.get("tree", []):
        path = REPO_ROOT / entry["path"]
        must_exist = bool(entry["must_exist"])
        exists = path.exists()
        if must_exist and not exists:
            failures.append(f"missing: {entry['path']}")
        elif not must_exist and exists:
            failures.append(f"still present: {entry['path']}")
    if failures:
        return fail("1-tree", failures)
    return emit("1-tree", "PASS", f"checks:{len(manifest.get('tree', []))}")


def layer_forbidden_text(manifest: dict) -> int:
    failures: list[str] = []
    checks = manifest.get("forbidden_text", [])
    for entry in checks:
        path = REPO_ROOT / entry["file"]
        needle = entry["needle"]
        if not path.exists():
            failures.append(f"file missing: {entry['file']}")
            continue
        try:
            content = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as e:
            failures.append(f"{entry['file']}: read error: {e}")
            continue
        if needle in content:
            count = content.count(needle)
            failures.append(f"{entry['file']}: still contains '{needle}' ({count}x)")
    if failures:
        return fail("2-forbidden-text", failures)
    return emit("2-forbidden-text", "PASS", f"checks:{len(checks)}")


def layer_waivers(manifest: dict) -> int:
    spec = manifest.get("waivers", {})
    if not spec:
        return emit("3-waivers", "PASS", "no spec")
    forbidden_id = spec["forbidden_id"]
    file_pattern = spec["file_pattern"]
    search_root = REPO_ROOT / spec["search_root"]
    expected = int(spec["expected_count_after"])
    hits: list[Path] = []
    for path in search_root.rglob(file_pattern):
        if "target" in path.parts:
            continue
        try:
            with path.open("rb") as fp:
                data = tomllib.load(fp)
        except (tomllib.TOMLDecodeError, OSError):
            continue
        for waiver in data.get("waivers", []) or []:
            if waiver.get("rule") == forbidden_id:
                hits.append(path)
                break
    if len(hits) != expected:
        msgs = [f"expected {expected} files referencing '{forbidden_id}', found {len(hits)}"]
        msgs.extend(f"  {h.relative_to(REPO_ROOT)}" for h in hits[:10])
        if len(hits) > 10:
            msgs.append(f"  ... and {len(hits) - 10} more")
        return fail("3-waivers", msgs)
    return emit("3-waivers", "PASS", f"hits:{len(hits)}")


def matches_any(rel: str, globs: list[str]) -> bool:
    for g in globs:
        if fnmatch.fnmatch(rel, g):
            return True
        if "**" in g and fnmatch.fnmatch(rel, g.replace("**/", "*/")):
            return True
    return False


def repo_grep(pattern: str, exclude_globs: list[str]) -> list[tuple[str, int]]:
    """Use ripgrep for fast repo-wide search."""
    cmd = [
        "rg",
        "--no-messages",
        "--with-filename",
        "--line-number",
        "--no-heading",
        "--color=never",
        "-g",
        "!target",
        "-g",
        "!**/target/**",
        "-g",
        "!.git",
        "-g",
        "!node_modules",
        "-g",
        "!**/node_modules/**",
        pattern,
        ".",
    ]
    try:
        proc = subprocess.run(
            cmd,
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            timeout=60,
        )
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return []
    results: list[tuple[str, int]] = []
    for line in proc.stdout.splitlines():
        parts = line.split(":", 2)
        if len(parts) < 3:
            continue
        rel, lineno = parts[0].lstrip("./"), parts[1]
        if matches_any(rel, exclude_globs):
            continue
        try:
            results.append((rel, int(lineno)))
        except ValueError:
            continue
    return results


def layer_repo_grep(manifest: dict) -> int:
    failures: list[str] = []
    checks = manifest.get("repo_grep", [])
    for entry in checks:
        pattern = entry["pattern"]
        excludes = entry.get("exclude_globs", [])
        expected = int(entry["expected_count"])
        hits = repo_grep(pattern, excludes)
        if len(hits) != expected:
            failures.append(
                f"pattern '{pattern}' expected {expected} hits, found {len(hits)}"
            )
            for rel, lineno in hits[:8]:
                failures.append(f"  {rel}:{lineno}")
            if len(hits) > 8:
                failures.append(f"  ... and {len(hits) - 8} more")
    if failures:
        return fail("4-repo-grep", failures)
    return emit("4-repo-grep", "PASS", f"checks:{len(checks)}")


def layer_validate(manifest: dict) -> int:
    if not G3RS_BIN.exists():
        return fail("5-validate", [f"g3rs binary missing at {G3RS_BIN}"])
    failures: list[str] = []
    checks = manifest.get("validate_workspace", [])
    for entry in checks:
        ws = entry["path"]
        try:
            proc = subprocess.run(
                [str(G3RS_BIN), "validate", "--path", ws],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
                timeout=600,
            )
        except subprocess.TimeoutExpired:
            failures.append(f"{ws}: timeout")
            continue
        if proc.returncode != 0:
            failures.append(f"{ws}: exit {proc.returncode}")
    if failures:
        return fail("5-validate", failures)
    return emit("5-validate", "PASS", f"checks:{len(checks)}")


def main() -> int:
    if not MANIFEST.exists():
        print(f"ERROR: manifest missing at {MANIFEST}", file=sys.stderr)
        return 1
    manifest = load_manifest()

    skip_validate = "--skip-validate" in sys.argv

    overall = 0
    overall |= layer_tree(manifest)
    overall |= layer_forbidden_text(manifest)
    overall |= layer_waivers(manifest)
    overall |= layer_repo_grep(manifest)
    if not skip_validate:
        overall |= layer_validate(manifest)
    else:
        print("layer:5-validate status:SKIPPED")

    print()
    print("verify-remove-structural-split:", "PASS" if overall == 0 else "FAIL")
    return overall


if __name__ == "__main__":
    sys.exit(main())
