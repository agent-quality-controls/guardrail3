#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
RULE_FIXTURE_ROOT = REPO_ROOT / "behavior/fixtures/g3rs-rule"
REPLAY_MANIFEST = (
    REPO_ROOT / ".plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
)
ORACLE_SCRIPTS = {
    "exact": "scripts/behavior/reduce-g3rs-fixture-oracle.py",
    "rule-set": "scripts/behavior/reduce-g3rs-fixture-rule-set-oracle.py",
}


def main() -> int:
    args = parse_args()
    roots = fixture_roots()
    if args.case:
        roots = [root for root in roots if root.name == args.case]
    if args.limit is not None:
        roots = roots[: args.limit]
    if not roots:
        raise SystemExit("no broken fixture roots matched")

    report_rows = []
    for root in roots:
        row = reduce_root(root, args.max_oracle_calls, args.oracle)
        report_rows.append(row)

    print(json.dumps({"reduced": report_rows}, indent=2, sort_keys=True))
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--case", help="Reduce one fixture case directory name")
    parser.add_argument("--limit", type=int, help="Reduce the first N broken fixture roots")
    parser.add_argument("--max-oracle-calls", type=int, help="Pass a shared oracle-call cap to fixture3")
    parser.add_argument("--oracle", choices=sorted(ORACLE_SCRIPTS), default="exact")
    return parser.parse_args()


def fixture_roots() -> list[Path]:
    roots = []
    for fixture in sorted(RULE_FIXTURE_ROOT.glob("*/*/fixture.toml")):
        if "R00-clean-golden" not in fixture.parent.name:
            roots.append(fixture.parent)
    return roots


def reduce_root(root: Path, max_oracle_calls: int | None, oracle: str) -> dict[str, object]:
    rel = root.relative_to(REPO_ROOT)
    scratch = REPO_ROOT / ".fixture3/reduce-g3rs-broken-family-rule-fixtures" / rel
    backup = scratch / "backup"
    work_dir = scratch / "work"
    approved_dir = scratch / "approved"
    manifest = scratch / "manifest.fixture3.yaml"

    if scratch.exists():
        shutil.rmtree(scratch)
    scratch.mkdir(parents=True)
    shutil.copytree(root, backup)
    approved_dir.mkdir(parents=True)
    oracle_script = ORACLE_SCRIPTS[oracle]
    write_single_fixture_approved(root / "fixture.toml", approved_dir / "approved.normalized.json", oracle_script)
    write_reduce_manifest(root, approved_dir, work_dir, manifest, oracle_script)

    command = [
        "fixture3",
        "reduce",
        "--manifest",
        str(manifest.relative_to(REPO_ROOT)),
        "--suite",
        "reduce-case",
        "--fixture-root",
        str(rel),
        "--work-dir",
        str(work_dir.relative_to(REPO_ROOT)),
    ]
    if max_oracle_calls is not None:
        command.extend(["--max-oracle-calls", str(max_oracle_calls)])
    completed = run(command)
    report = json.loads(completed.stdout)
    if report["guarantee"] == "incomplete:baseline-not-interesting":
        raise SystemExit(f"reducer baseline did not match for {rel}")

    try:
        apply_report(root, report)
        verify_single_fixture_behavior(root, approved_dir / "approved.normalized.json", oracle_script)
    except BaseException:
        restore_root(root, backup)
        raise
    changed = bool(report["removed_files"] or report["removed_directories"])
    return {
        "fixture": rel.as_posix(),
        "guarantee": report["guarantee"],
        "oracle_calls": report["oracle_calls"],
        "removed_files": len(report["removed_files"]),
        "removed_directories": len(report["removed_directories"]),
        "changed": changed,
    }


def write_single_fixture_approved(fixture: Path, output: Path, oracle_script: str) -> None:
    fixture_root = fixture.parent
    fixture_args = [
        str(path.relative_to(REPO_ROOT))
        for path in sorted(fixture_root.rglob("*"))
        if path.is_file()
    ]
    completed = run(
        [
            "python3",
            oracle_script,
            "--manifest",
            str(REPLAY_MANIFEST.relative_to(REPO_ROOT)),
            *fixture_args,
        ]
    )
    output.write_text(completed.stdout, encoding="utf-8")


def write_reduce_manifest(
    root: Path,
    approved_dir: Path,
    work_dir: Path,
    manifest: Path,
    oracle_script: str,
) -> None:
    root_rel = root.relative_to(REPO_ROOT).as_posix()
    approved_rel = approved_dir.relative_to(REPO_ROOT).as_posix()
    received_rel = (work_dir / "received").relative_to(REPO_ROOT).as_posix()
    diff_rel = (work_dir / "diff").relative_to(REPO_ROOT).as_posix()
    replay_manifest_rel = REPLAY_MANIFEST.relative_to(REPO_ROOT).as_posix()
    manifest.write_text(
        "\n".join(
            [
                "version: 1",
                "suites:",
                "  reduce-case:",
                "    fixtures:",
                f'      - "{root_rel}/**/*"',
                "    command:",
                "      argv:",
                '        - "python3"',
                f'        - "{oracle_script}"',
                '        - "--manifest"',
                f'        - "{replay_manifest_rel}"',
                '        - "{fixtures}"',
                "      ok_exit_codes:",
                "        - 0",
                "    storage:",
                f'      approved_dir: "{approved_rel}"',
                f'      received_dir: "{received_rel}"',
                f'      diff_dir: "{diff_rel}"',
                "",
            ]
        ),
        encoding="utf-8",
    )


def apply_report(root: Path, report: dict[str, object]) -> None:
    for raw_dir in sorted(report["removed_directories"], key=lambda item: len(str(item)), reverse=True):
        path = root / str(raw_dir)
        if path.exists():
            shutil.rmtree(path)
    for raw_file in report["removed_files"]:
        path = root / str(raw_file)
        if path.exists():
            path.unlink()


def restore_root(root: Path, backup: Path) -> None:
    if root.exists():
        shutil.rmtree(root)
    shutil.copytree(backup, root)


def verify_single_fixture_behavior(root: Path, approved: Path, oracle_script: str) -> None:
    fixture_args = [
        str(path.relative_to(REPO_ROOT))
        for path in sorted(root.rglob("*"))
        if path.is_file()
    ]
    completed = run(
        [
            "python3",
            oracle_script,
            "--manifest",
            str(REPLAY_MANIFEST.relative_to(REPO_ROOT)),
            *fixture_args,
        ]
    )
    if json.loads(completed.stdout) != json.loads(approved.read_text(encoding="utf-8")):
        raise SystemExit(f"behavior changed after reduction: {root.relative_to(REPO_ROOT)}")


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise SystemExit(f"command failed: {' '.join(command)}")
    return completed


if __name__ == "__main__":
    sys.exit(main())
