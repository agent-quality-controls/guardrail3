#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / ".plans/2026-05-19-103450-universal-waiver-policy.md.manifest.toml"


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []
    failures.extend(check_package(manifest))
    failures.extend(check_public_api(manifest))
    failures.extend(check_parser_schemas(manifest))
    failures.extend(check_forbidden_types(manifest))
    failures.extend(check_family_matchers(manifest))
    failures.extend(check_rule_contract(manifest))
    failures.extend(check_fixture_contract())

    if failures:
        print("FAIL universal waiver policy")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print("PASS universal waiver policy")
    return 0


def load_manifest() -> dict:
    with MANIFEST.open("rb") as file:
        return tomllib.load(file)


def read(rel_path: str) -> str:
    return (ROOT / rel_path).read_text(encoding="utf-8")


def exists(rel_path: str) -> bool:
    return (ROOT / rel_path).exists()


def check_package(manifest: dict) -> list[str]:
    failures: list[str] = []
    for package in manifest.get("package", []):
        path = package["path"]
        if not exists(path):
            failures.append(f"missing package path: {path}")
            continue
        cargo_toml = Path(path) / "Cargo.toml"
        if not exists(cargo_toml.as_posix()):
            failures.append(f"missing package Cargo.toml: {cargo_toml.as_posix()}")
    return failures


def check_public_api(manifest: dict) -> list[str]:
    failures: list[str] = []
    source_root = ROOT / "packages/parsers/g3-guardrail-toml-types/src"
    source = "\n".join(path.read_text(encoding="utf-8") for path in sorted(source_root.rglob("*.rs")))
    patterns = {
        "WaiverConfig": r"pub struct WaiverConfig\b",
        "WaiverMatch": r"pub struct WaiverMatch\b",
        "WaiverReason": r"pub struct WaiverReason\b",
        "find_waiver_reason": r"pub fn find_waiver_reason\b",
        "has_waiver": r"pub fn has_waiver\b",
    }
    for api in manifest.get("public_api", []):
        if api["crate"] != "g3-guardrail-toml-types":
            failures.append(f"unexpected public API crate in manifest: {api['crate']}")
            continue
        for symbol in api["symbols"]:
            pattern = patterns.get(symbol)
            if pattern is None:
                failures.append(f"verifier has no public API pattern for {symbol}")
            elif re.search(pattern, source) is None:
                failures.append("shared waiver API missing symbol " + symbol)
    return failures


def check_parser_schemas(manifest: dict) -> list[str]:
    failures: list[str] = []
    parser_paths = {
        "g3rs-toml-parser": {
            "cargo": "packages/parsers/g3rs-toml-parser/crates/types/Cargo.toml",
            "source": "packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs",
        },
        "g3ts-toml-parser": {
            "cargo": "packages/parsers/g3ts-toml-parser/crates/types/Cargo.toml",
            "source": "packages/parsers/g3ts-toml-parser/crates/types/src/guardrail3_ts_toml.rs",
        },
    }
    for schema in manifest.get("parser_schema", []):
        parser = schema["parser"]
        paths = parser_paths[parser]
        cargo = read(paths["cargo"])
        source = read(paths["source"])
        if 'g3-guardrail-toml-types' not in cargo:
            failures.append(f"{parser} types crate does not depend on g3-guardrail-toml-types")
        if "pub use g3_guardrail_toml_types::WaiverConfig;" not in source:
            failures.append(f"{parser} does not re-export shared WaiverConfig")
        if re.search(r"pub struct WaiverConfig\b", source):
            failures.append(f"{parser} still defines local WaiverConfig")
        if schema["must_have_field"] == "waivers" and "pub waivers: Vec<WaiverConfig>" not in source:
            failures.append(f"{parser} root schema does not expose waivers: Vec<WaiverConfig>")
    return failures


def check_forbidden_types(manifest: dict) -> list[str]:
    failures: list[str] = []
    for item in manifest.get("forbidden_type", []):
        source = read(item["path"])
        name = item["name"]
        if re.search(rf"\b{name}\b", source):
            failures.append(f"forbidden waiver type remains in {item['path']}: {name}")
    return failures


def check_family_matchers(manifest: dict) -> list[str]:
    failures: list[str] = []
    code_roots = [
        ROOT / "packages/rs",
        ROOT / "packages/parsers/g3rs-toml-parser",
        ROOT / "packages/parsers/g3ts-toml-parser",
    ]
    shared_root = (ROOT / "packages/parsers/g3-guardrail-toml-types/src").resolve()
    forbidden_patterns = [
        r"waiver\.rule\s*==",
        r"entry\.rule\s*==",
        r"\.rule\s*==\s*target\.rule",
    ]
    for root in code_roots:
        for path in root.rglob("*.rs"):
            if path.resolve().is_relative_to(shared_root):
                continue
            source = path.read_text(encoding="utf-8")
            for pattern in forbidden_patterns:
                if re.search(pattern, source):
                    failures.append(f"local waiver matcher remains in {path.relative_to(ROOT).as_posix()}")
    shared_consumers = {
        "cargo": "packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs",
        "clippy": "packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs",
        "code": "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs",
        "fmt": "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/ignore_escape_hatch/rule.rs",
        "garde": "packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs",
        "arch": "packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/dependency_count_split.rs",
        "apparch": "packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/patch_replace_bypass.rs",
        "deps": "packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/direct_dependency_cap/rule.rs",
    }
    for consumer in manifest.get("waiver_consumer", []):
        family = consumer["family"]
        source = read(shared_consumers[family])
        if "find_waiver_reason" not in source and "has_waiver" not in source:
            failures.append(f"{family} waiver consumer does not call shared matcher")
        if "g3_guardrail_toml_types" not in source:
            failures.append(f"{family} waiver consumer does not import shared waiver package")
    return failures


def check_rule_contract(manifest: dict) -> list[str]:
    failures: list[str] = []
    source = read("packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/direct_dependency_cap/rule.rs")
    for contract in manifest.get("rule_contract", []):
        if contract["rule"] != "g3rs-deps/direct-dependency-cap":
            failures.append(f"unexpected rule contract: {contract['rule']}")
            continue
        if contract["selector"] not in source:
            failures.append("deps direct dependency cap selector is missing")
        if "find_waiver_reason" not in source:
            failures.append("deps direct dependency cap does not use shared matcher")
        if "warn(" not in source or '"direct dependency cap waived"' not in source:
            failures.append("deps direct dependency cap does not emit waived warning")
        if "error(" not in source or '"too many direct dependencies"' not in source:
            failures.append("deps direct dependency cap does not emit unwaived error")
    return failures


def check_fixture_contract() -> list[str]:
    failures: list[str] = []
    fixture = ROOT / "behavior/fixtures/g3rs-rule/deps/deps-R22-direct-dependency-cap-waived"
    if not fixture.exists():
        return ["missing deps waived direct dependency cap fixture"]
    guardrail = (fixture / "repo/guardrail3-rs.toml").read_text(encoding="utf-8")
    metadata = (fixture / "fixture.toml").read_text(encoding="utf-8")
    for required in (
        'rule = "g3rs-deps/direct-dependency-cap"',
        'file = "Cargo.toml"',
        'selector = "unique-direct-dependency-count"',
        "reason = ",
    ):
        if required not in guardrail:
            failures.append(f"deps waived fixture guardrail config missing {required}")
    if 'expected_exit = "zero"' not in metadata:
        failures.append("deps waived fixture must expect zero exit")
    if "direct_dependency_count_above_cap_waived" not in metadata:
        failures.append("deps waived fixture metadata does not identify waived cap behavior")
    return failures


if __name__ == "__main__":
    sys.exit(main())
