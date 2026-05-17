#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

from replay_common import REPO_ROOT


FINDING_RE = re.compile(r"^\[(Error|Warn|Info)\] (g3rs-[^ ]+)")


def main() -> int:
    args = sys.argv[1:]
    fixture_paths = [arg for arg in args if Path(arg).name == "fixture.toml"]
    if len(fixture_paths) != 1:
        raise SystemExit(f"expected exactly one fixture.toml, found {len(fixture_paths)}")

    completed = subprocess.run(
        [
            sys.executable,
            str(REPO_ROOT / "scripts/behavior/fixture3-g3rs-replay.py"),
            *args[:2],
            *fixture_paths,
        ],
        text=True,
        stdout=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        return int(completed.returncode)

    payload = json.loads(completed.stdout)
    records = []
    for record in payload.get("records", []):
        stdout = record.get("stdout", "")
        if not isinstance(stdout, str):
            stdout = ""
        findings: dict[str, set[str]] = {}
        for line in stdout.splitlines():
            match = FINDING_RE.match(line)
            if match:
                severity, rule_id = match.groups()
                findings.setdefault(severity, set()).add(rule_id)
        exit_code = record.get("exit_code")
        records.append(
            {
                "exit": "zero" if exit_code == 0 else "nonzero",
                "fixture_id": "fixture-root",
                "rules": {key: sorted(value) for key, value in sorted(findings.items())},
            }
        )

    print(json.dumps({"records": records}, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
