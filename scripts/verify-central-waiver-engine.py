#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = REPO_ROOT / ".plans/2026-05-19-141731-central-waiver-engine.md.manifest.toml"


def main() -> int:
    manifest = load_manifest()
    errors: list[str] = []
    errors.extend(check_tree(manifest))
    errors.extend(check_public_api(manifest))
    errors.extend(check_schema())
    errors.extend(check_runner_contracts(manifest))
    errors.extend(check_forbidden_imports(manifest))
    errors.extend(check_forbidden_config_keys())
    errors.extend(check_fixture_contracts(manifest))
    errors.extend(check_legacy_parser_removed())

    if errors:
        for error in errors:
            print(f"FAIL {error}")
        return 1
    print("PASS central waiver engine manifest")
    return 0


def load_manifest() -> dict:
    with MANIFEST_PATH.open("rb") as file:
        return tomllib.load(file)


def rel(path: Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def read(path: str) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def check_tree(manifest: dict) -> list[str]:
    errors = []
    for entry in manifest.get("tree", []):
        path = REPO_ROOT / entry["path"]
        if not path.exists():
            errors.append(f"missing required path: {entry['path']}")
    return errors


def check_public_api(manifest: dict) -> list[str]:
    errors = []
    crate_sources = {
        "guardrail3-waivers": "packages/shared/guardrail3-waivers/src/lib.rs",
        "guardrail3-check-types": "packages/shared/guardrail3-check-types/src/result.rs",
    }
    for api in manifest.get("public_api", []):
        source_path = crate_sources[api["crate"]]
        source = read(source_path)
        for type_name in api.get("types", []):
            if not exposes_token(source, type_name):
                errors.append(f"{source_path} does not expose type {type_name}")
        for function_name in api.get("functions", []):
            if not exposes_token(source, function_name):
                errors.append(f"{source_path} does not expose function {function_name}")
    return errors


def exposes_token(source: str, token: str) -> bool:
    escaped = re.escape(token)
    return bool(
        re.search(rf"\b(?:pub\s+)?(?:struct|enum|type|fn)\s+{escaped}\b", source)
        or re.search(rf"pub use [^;]*\b{escaped}\b", source, flags=re.DOTALL)
    )


def check_schema() -> list[str]:
    source_path = "packages/shared/guardrail3-waivers/src/waiver.rs"
    source = read(source_path)
    errors = []
    required_fields = ["rule", "subject", "selector", "reason"]
    for field in required_fields:
        if not re.search(rf"\b{field}\s*:\s*String\b", source):
            errors.append(f"{source_path} missing WaiverConfig.{field}: String")
    if "pub file:" in source or "flatten" in source or "extra:" in source:
        errors.append(f"{source_path} still accepts legacy or unknown waiver fields")
    if "#[serde(deny_unknown_fields)]" not in source:
        errors.append(f"{source_path} must deny unknown waiver fields")
    return errors


def check_runner_contracts(manifest: dict) -> list[str]:
    errors = []
    for contract in manifest.get("runner_contract", []):
        source = read(contract["path"])
        if contract["must_call"] not in source:
            errors.append(f"{contract['path']} does not call {contract['must_call']}")
        if contract["must_parse"] not in source:
            errors.append(f"{contract['path']} does not parse config with {contract['must_parse']}")
    return errors


def check_forbidden_imports(manifest: dict) -> list[str]:
    errors = []
    for rule in manifest.get("forbidden_import", []):
        scope = REPO_ROOT / rule["scope"]
        for path in rust_sources(scope):
            text = path.read_text(encoding="utf-8")
            for pattern in rule["patterns"]:
                if pattern in text:
                    errors.append(f"{rel(path)} contains forbidden waiver pattern `{pattern}`")
    return errors


def rust_sources(scope: Path) -> list[Path]:
    if not scope.exists():
        return []
    return [
        path
        for path in scope.rglob("*")
        if path.is_file()
        and "target" not in path.parts
        and (path.suffix == ".rs" or path.name == "Cargo.toml")
    ]


def check_forbidden_config_keys() -> list[str]:
    errors = []
    for path in (REPO_ROOT / "behavior/fixtures").rglob("guardrail3-*.toml"):
        text = path.read_text(encoding="utf-8")
        if re.search(r"(?m)^\s*file\s*=", text):
            errors.append(f"{rel(path)} still uses waiver file key")
    return errors


def check_fixture_contracts(manifest: dict) -> list[str]:
    errors = []
    for contract in manifest.get("fixture_contract", []):
        path = REPO_ROOT / contract["path"]
        if not path.exists():
            errors.append(f"missing fixture contract path: {contract['path']}")
            continue
        if contract.get("must_have_waived_fixture"):
            if not any("subject =" in file.read_text(encoding="utf-8") for file in path.rglob("guardrail3-ts.toml")):
                errors.append(f"{contract['path']} has no G3TS waiver using subject")
        for expected in contract.get("must_contain", []):
            if not any(expected in file.read_text(encoding="utf-8") for file in path.rglob("*") if file.is_file()):
                errors.append(f"{contract['path']} does not contain {expected}")
    return errors


def check_legacy_parser_removed() -> list[str]:
    path = REPO_ROOT / "packages/parsers/g3-guardrail-toml-types"
    if path.exists():
        return [f"{rel(path)} still exists"]
    return []


if __name__ == "__main__":
    sys.exit(main())
