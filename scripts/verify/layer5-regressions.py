#!/usr/bin/env python3
"""Layer 5: empirical regression injection scenarios.

For each [[regression_scenario]]:
  1. Backup the target file (md5).
  2. Inject the regression per the `inject` action.
  3. Run the named command.
  4. Assert exit code matches `expected_exit`.
  5. If exit_code matches, assert the named rule fired at named severity.
  6. Restore the target file.
  7. md5 round-trip.

For each [[commit_gate_scenario]]:
  Same plus stage a file and run `bash .githooks/pre-commit` directly.

Failures aggregate.
"""

from __future__ import annotations

import hashlib
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, load_manifest, section


def md5(path: Path) -> str:
    h = hashlib.md5()
    h.update(path.read_bytes())
    return h.hexdigest()


def inject(content: str, action: str, entry: dict) -> str:
    if action == "replace_token":
        return content.replace(entry["inject_from"], entry["inject_to"])
    if action == "replace_token_in_ts_loop":
        # Replace only inside the TS discovery loop. Heuristic: replace
        # second occurrence of inject_from (assumes RS loop comes first).
        first = content.find(entry["inject_from"])
        if first < 0:
            return content
        second = content.find(entry["inject_from"], first + 1)
        if second < 0:
            return content
        return (
            content[:second]
            + entry["inject_to"]
            + content[second + len(entry["inject_from"]) :]
        )
    if action == "append_before_final_echo":
        text = entry["inject_text"]
        marker = 'echo "All pre-commit checks passed."'
        return content.replace(
            marker, f"{text}\n{marker}", 1
        )
    if action == "prepend_before_rust_loop":
        text = entry["inject_text"]
        marker = "# --- Rust discovery loop ---"
        out = content.replace(marker, f"{text}\n{marker}", 1)
        if entry.get("inject_replace_loop_scope"):
            # Find the assigned variable name (LHS of inject_text) and replace
            # the Rust verifier scope with $VAR.
            import re as _re
            m = _re.match(r'^([A-Z_][A-Z0-9_]*)=', text)
            if m:
                var = m.group(1)
                # Replace only the first occurrence (RS loop comes before TS).
                first = out.find('--path "$unit"')
                if first >= 0:
                    out = out[:first] + f"--path \"${var}\"" + out[first + len('--path "$unit"') :]
        return out
    if action == "prepend_before_rust_loop_invocation":
        text = entry["inject_text"]
        # Insert before the `if [ -n "$RUST_UNIQUE_UNITS" ]` block.
        marker = 'if [ -n "$RUST_UNIQUE_UNITS" ]'
        return content.replace(marker, f"{text}\n{marker}", 1)
    if action == "prepend_before_ts_loop":
        text = entry["inject_text"]
        marker = "# --- TS discovery loop ---"
        out = content.replace(marker, f"{text}\n{marker}", 1)
        if entry.get("inject_replace_loop_scope_ts"):
            import re as _re
            m = _re.match(r'^([A-Z_][A-Z0-9_]*)=', text)
            if m:
                var = m.group(1)
                # The TS loop's --scope appears AFTER the RS loop's. Replace second occurrence.
                first = out.find('--path "$unit"')
                if first >= 0:
                    second = out.find('--path "$unit"', first + 1)
                    if second >= 0:
                        out = out[:second] + f"--path \"${var}\"" + out[second + len('--path "$unit"') :]
        return out
    if action == "prepend_before_ts_loop_invocation":
        text = entry["inject_text"]
        marker = 'if [ -n "$TS_UNIQUE_UNITS" ]'
        return content.replace(marker, f"{text}\n{marker}", 1)
    if action == "delete_line_matching":
        pattern = entry["inject_pattern"]
        return "\n".join(
            line for line in content.splitlines() if pattern not in line
        ) + "\n"
    if action == "delete_block_matching":
        start = entry["inject_pattern_start"]
        end = entry["inject_pattern_end"]
        lines = content.splitlines()
        out: list[str] = []
        skipping = False
        for line in lines:
            if not skipping and start in line:
                skipping = True
                continue
            # Match `end` only as the trimmed line (avoids "fi" inside "file").
            if skipping and line.strip() == end:
                skipping = False
                continue
            if not skipping:
                out.append(line)
        return "\n".join(out) + "\n"
    raise ValueError(f"unknown inject action: {action}")


def find_binary(name: str) -> Path | None:
    rel = REPO_ROOT / "apps" / name / "target" / "release" / name
    if rel.exists():
        return rel
    fallback = shutil.which(name)
    return Path(fallback) if fallback else None


def run_validate_repo(binary_name: str) -> tuple[int, str]:
    bin_path = find_binary(binary_name)
    if bin_path is None:
        return (127, f"binary not found: {binary_name}")
    result = subprocess.run(
        [str(bin_path), "validate-repo"],
        cwd=str(REPO_ROOT),
        capture_output=True,
        text=True,
    )
    return (result.returncode, (result.stdout or "") + (result.stderr or ""))


def run_hook() -> tuple[int, str]:
    hook = REPO_ROOT / ".githooks" / "pre-commit"
    result = subprocess.run(
        ["bash", str(hook)],
        cwd=str(REPO_ROOT),
        capture_output=True,
        text=True,
    )
    return (result.returncode, (result.stdout or "") + (result.stderr or ""))


def find_finding_with(out: str, rule_id: str, severity: str) -> bool:
    pattern = re.compile(
        rf"^\[{re.escape(severity)}\]\s+{re.escape(rule_id)}",
        re.MULTILINE,
    )
    return bool(pattern.search(out))


def run_scenario(entry: dict) -> tuple[bool, str]:
    target = REPO_ROOT / entry["target"]
    if not target.exists():
        return (False, f"target missing: {entry['target']}")
    original = target.read_bytes()
    pre_md5 = md5(target)
    try:
        injected = inject(original.decode(), entry["inject"], entry)
        target.write_text(injected)
        binary = entry["command"].split()[0]
        sub = entry["command"].split()[1]
        if sub != "validate-repo":
            return (False, f"only validate-repo supported, got: {entry['command']}")
        rc, out = run_validate_repo(binary)
    finally:
        target.write_bytes(original)
        post_md5 = md5(target)
        if post_md5 != pre_md5:
            return (
                False,
                f"md5 mismatch after restore: {pre_md5} != {post_md5}",
            )

    if rc != entry["expected_exit"]:
        return (
            False,
            f"exit code mismatch: expected {entry['expected_exit']}, got {rc}",
        )
    if not find_finding_with(
        out, entry["expected_rule"], entry["expected_severity"]
    ):
        snippet = out[-400:].replace("\n", " | ")
        return (
            False,
            f"expected finding not present: [{entry['expected_severity']}] {entry['expected_rule']}; tail: {snippet}",
        )
    return (True, "")


def run_commit_gate(entry: dict) -> tuple[bool, str]:
    target = REPO_ROOT / entry["inject_target"]
    if not target.exists():
        return (False, f"target missing: {entry['inject_target']}")

    # Create a temp Rust file inside an adopted Rust workspace, stage it, run
    # the hook, then unstage and remove. We use a path inside apps/guardrail3-rs
    # so the discovery loop has something to route.
    stage_dir = REPO_ROOT / "apps" / "guardrail3-rs" / "crates"
    if not stage_dir.exists():
        return (False, f"stage dir missing: {stage_dir}")
    stage_file = stage_dir / "_verify_gate_marker.rs"

    original = target.read_bytes()
    pre_md5 = md5(target)
    try:
        stage_file.write_text("// verifier gate scenario marker\n")
        rc_stage = subprocess.run(
            ["git", "add", "--intent-to-add", str(stage_file)],
            cwd=str(REPO_ROOT),
            capture_output=True,
            text=True,
        )
        if rc_stage.returncode != 0:
            return (False, f"git add --intent-to-add failed: {rc_stage.stderr}")
        # Now stage the actual content.
        rc_full = subprocess.run(
            ["git", "add", str(stage_file)],
            cwd=str(REPO_ROOT),
            capture_output=True,
            text=True,
        )
        if rc_full.returncode != 0:
            return (False, f"git add failed: {rc_full.stderr}")

        injected = inject(original.decode(), entry["inject"], entry)
        target.write_text(injected)
        rc, out = run_hook()
    finally:
        target.write_bytes(original)
        post_md5 = md5(target)
        # Unstage and remove the temp file.
        subprocess.run(
            ["git", "rm", "--cached", "-f", str(stage_file)],
            cwd=str(REPO_ROOT),
            capture_output=True,
            text=True,
        )
        if stage_file.exists():
            stage_file.unlink()
        if post_md5 != pre_md5:
            return (False, f"md5 mismatch after restore: {pre_md5} != {post_md5}")
    if rc != entry["expected_hook_exit"]:
        return (
            False,
            f"hook exit mismatch: expected {entry['expected_hook_exit']}, got {rc}; tail: {out[-200:]!r}",
        )
    return (True, "")


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "regression_scenario"):
        ok, msg = run_scenario(entry)
        if not ok:
            failures.append(f"{entry['id']}: {msg}")

    for entry in section(manifest, "commit_gate_scenario"):
        ok, msg = run_commit_gate(entry)
        if not ok:
            failures.append(f"{entry['id']} (commit-gate): {msg}")

    if failures:
        print("layer:5-regressions status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    total = len(section(manifest, "regression_scenario")) + len(
        section(manifest, "commit_gate_scenario")
    )
    print(f"layer:5-regressions status:PASS scenarios:{total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
