#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path


REPO = Path(__file__).resolve().parents[1]


def main() -> int:
    failures: list[str] = []
    assert_no_symbol(
        REPO / "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src",
        "is_astro_workspace",
        failures,
    )
    assert_no_symbol(
        REPO / "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src",
        "default_disabled_families",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs",
        "pub enum GuardrailConfigError",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs",
        "pub fn disabled_families(package_root: &Path) -> Result<DisabledFamilies, GuardrailConfigError>",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs",
        "pub enum GuardrailConfigError",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs",
        "enabled_families: &[SupportedFamily]",
        failures,
    )
    assert_not_contains(
        REPO / "apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs",
        "disabled: &[SupportedFamily]",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/run.rs",
        "fn hook_contracts(enabled_families: &[SupportedFamily])",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs",
        "fn rust_hook_requirements(",
        failures,
    )
    assert_contains(
        REPO / "apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs",
        "enabled_families: &[SupportedFamily]",
        failures,
    )
    assert_no_active_pattern(r"\bdefault framework\b", failures)

    if failures:
        print("FAIL family enablement contract")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("PASS family enablement contract")
    return 0


def assert_no_symbol(root: Path, symbol: str, failures: list[str]) -> None:
    for path in root.rglob("*.rs"):
        text = path.read_text(encoding="utf-8")
        if re.search(rf"\b{re.escape(symbol)}\b", text):
            failures.append(f"{relative(path)} still contains symbol {symbol}")


def assert_contains(path: Path, needle: str, failures: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    if needle not in text:
        failures.append(f"{relative(path)} missing `{needle}`")


def assert_not_contains(path: Path, needle: str, failures: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    if needle in text:
        failures.append(f"{relative(path)} still contains `{needle}`")


def assert_no_active_pattern(pattern: str, failures: list[str]) -> None:
    roots = [
        REPO / "apps/guardrail3-ts",
        REPO / "apps/guardrail3-rs",
        REPO / "packages/ts",
        REPO / "packages/rs",
    ]
    compiled = re.compile(pattern)
    for root in roots:
        for path in root.rglob("*.rs"):
            if compiled.search(path.read_text(encoding="utf-8")):
                failures.append(f"{relative(path)} matches forbidden pattern {pattern}")


def relative(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


if __name__ == "__main__":
    sys.exit(main())
