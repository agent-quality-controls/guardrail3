#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import sys
import tomllib


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / ".plans/2026-05-18-160504-g3ts-tool-family-hardening.md.manifest.toml"


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def fail(message: str) -> None:
    print(f"FAIL: {message}")
    raise SystemExit(1)


def require_file(path: str) -> None:
    if not (ROOT / path).is_file():
        fail(f"missing file: {path}")


def require_text(path: str, text: str) -> None:
    content = read(path)
    if text not in content:
        fail(f"{path} does not contain expected text: {text}")


def forbid_text(path: str, text: str) -> None:
    content = read(path)
    if text in content:
        fail(f"{path} contains forbidden text: {text}")


def verify_manifest_loads() -> None:
    with MANIFEST.open("rb") as handle:
        manifest = tomllib.load(handle)
    if len(manifest.get("family", [])) != 3:
        fail("manifest must define exactly three family work scopes")
    if len(manifest.get("rule_contract", [])) < 15:
        fail("manifest rule contract inventory is incomplete")


def verify_generated_hook() -> None:
    init_rs = "apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs"
    execute_rs = "apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs"
    hook = ".githooks/pre-commit.d/g3ts"
    require_text(init_rs, "validation_staged_files=$(printf")
    require_text(init_rs, "grep -vE '^behavior/fixtures/'")
    require_text(init_rs, "printf '%s\\n' \"$validation_staged_files\" | grep -qE '(^|/)package\\.json$'")
    require_text(init_rs, 'ts_relevant_files=$(echo "$validation_staged_files"')
    require_text(init_rs, "(^|/)\\.syncpackrc$")
    require_text(init_rs, "(^|/)tsconfig[^/]*\\.json$")
    require_text(init_rs, "(^|/)prettier\\.config\\.[^/]+$")
    require_text(init_rs, "(^|/)\\.prettierrc(\\.[^/]+)?$")
    require_text(init_rs, "(^|/)cspell\\.json$")
    require_text(init_rs, "(^|/)\\.cspell\\.json$")
    require_text(init_rs, "(^|/)cspell\\.config\\.[^/]+$")
    require_text(init_rs, "(^|/)cspell\\.ya?ml$")
    require_text(hook, "validation_staged_files=$(printf")
    require_text(hook, "grep -vE '^behavior/fixtures/'")
    require_text(hook, "printf '%s\\n' \"$validation_staged_files\" | grep -qE '(^|/)package\\.json$'")
    require_text(hook, 'ts_relevant_files=$(echo "$validation_staged_files"')
    require_text(hook, "(^|/)\\.syncpackrc$")
    require_text(hook, "(^|/)tsconfig[^/]*\\.json$")
    require_text(hook, "(^|/)prettier\\.config\\.[^/]+$")
    require_text(hook, "(^|/)\\.prettierrc(\\.[^/]+)?$")
    require_text(hook, "(^|/)cspell\\.json$")
    require_text(hook, "(^|/)\\.cspell\\.json$")
    require_text(hook, "(^|/)cspell\\.config\\.[^/]+$")
    require_text(hook, "(^|/)cspell\\.ya?ml$")
    require_text(execute_rs, '".syncpackrc"')
    require_text(execute_rs, '"cspell.json"')
    require_text(execute_rs, '".cspell.json"')
    require_text(execute_rs, '"cspell.yaml"')
    require_text(execute_rs, '"cspell.yml"')
    require_text(execute_rs, "name.starts_with(\"tsconfig\")")
    require_text(execute_rs, "ext.eq_ignore_ascii_case(\"json\")")
    require_text(execute_rs, "name.starts_with(\"prettier.config.\")")
    require_text(execute_rs, "name.starts_with(\".prettierrc.\")")
    require_text(execute_rs, "name.starts_with(\"cspell.config.\")")
    forbid_text(init_rs, "printf '%s\\n' \"$staged_files\" | grep -qE '(^|/)package\\.json$'")
    forbid_text(hook, "printf '%s\\n' \"$staged_files\" | grep -qE '(^|/)package\\.json$'")


def verify_shared_parser_ownership() -> None:
    package_parser = "packages/parsers/package-json-parser/crates/runtime/src/parser.rs"
    syncpack_matcher = "packages/parsers/syncpack-config-parser/crates/runtime/src/matcher.rs"
    family_paths = [
        "packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/syncpack_prettier_pin.rs",
        "packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/syncpack_cspell_pin.rs",
        "packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/syncpack_type_coverage_pin.rs",
    ]
    ingestion_paths = [
        "packages/ts/fmt/g3ts-fmt-ingestion/crates/runtime/src/package.rs",
        "packages/ts/spelling/g3ts-spelling-ingestion/crates/runtime/src/package.rs",
        "packages/ts/typecov/g3ts-typecov-ingestion/crates/runtime/src/package.rs",
    ]
    require_text(package_parser, "pub fn dependency_declarations")
    require_text(package_parser, "pub fn specifier_type")
    require_text(syncpack_matcher, "pub fn first_matching_group_pins_dependency")
    require_text(syncpack_matcher, "pub fn pattern_list_matches")
    require_text(syncpack_matcher, "globset::Glob::new")
    for path in family_paths:
        require_text(path, "syncpack_config_parser::first_matching_group_pins_dependency")
        require_text(path, "SyncpackDependencyDeclarationRef")
        forbid_text(path, "fn first_matching_group_pins_dependency")
        forbid_text(path, "fn pattern_list_matches")
        forbid_text(path, "globset::Glob::new")
    for path in ingestion_paths:
        require_text(path, "package_json_parser::dependency_declarations")
        forbid_text(path, "fn specifier_type")
        forbid_text(path, "fn range_specifier_type")


def verify_fmt() -> None:
    run_rs = "packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/run.rs"
    lib_rs = "packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/lib.rs"
    common_rs = "packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/common.rs"
    roots_rs = "packages/ts/fmt/g3ts-fmt-ingestion/crates/runtime/src/roots.rs"
    fixture_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R00-clean-golden/repo/package.json"
    fixture_guardrail = "behavior/fixtures/g3ts-rule/fmt/fmt-R00-clean-golden/repo/guardrail3-ts.toml"
    targetless_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/targetless-check/package.json"
    stdin_filepath_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/stdin-filepath-targetless/package.json"
    find_config_path_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/find-config-path-targetless/package.json"
    ignored_first_match_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/ignored-first-match-syncpack/.syncpackrc"
    banned_first_match_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/banned-first-match-syncpack/.syncpackrc"
    unpinned_first_match_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/unpinned-first-match-syncpack/.syncpackrc"
    dependency_negation_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/dependency-negation-syncpack-miss/.syncpackrc"
    dependency_type_negation_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/dependency-type-negation-syncpack-miss/.syncpackrc"
    specifier_type_negation_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/specifier-type-negation-syncpack-miss/.syncpackrc"
    latest_specifier_syncpack_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/latest-specifier-syncpack-miss/package.json"
    bare_targetless_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/bare-targetless-check/package.json"
    or_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/or-fallback/package.json"
    scoped_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/scoped-syncpack-miss/.syncpackrc"
    specifier_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/specifier-syncpack-miss/.syncpackrc"
    prod_syncpack = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/prod-syncpack-miss/.syncpackrc"
    prod_syncpack_applies = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/prod-dependency-syncpack-applies/package.json"
    prod_syncpack_applies_config = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/prod-dependency-syncpack-applies/.syncpackrc"
    runner_direct_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/runner-direct/package.json"
    filtered_shortcut_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R30-validate-not-wired/repo/filtered-shortcut/package.json"
    indirect_script_package = "behavior/fixtures/g3ts-rule/fmt/fmt-R30-validate-not-wired/repo/indirect-script/package.json"
    hook_contract = "packages/ts/fmt/g3ts-fmt-hook-contract/src/contract.rs"
    fmt_roots = "packages/ts/fmt/g3ts-fmt-ingestion/crates/runtime/src/roots.rs"

    require_text(run_rs, "format_check")
    require_text(lib_rs, "format_check")
    require_text(common_rs, "format:check")
    require_text(common_rs, "--check")
    require_text(common_rs, "--cache")
    require_text(common_rs, "script_has_no_or_separator")
    require_text(common_rs, "prettier_args_have_target")
    require_text(common_rs, "--stdin-filepath")
    require_text(common_rs, "--find-config-path")
    require_text(common_rs, "original_command_starts_with(invocation, \"prettier\")")
    require_text(common_rs, "invocation_uses_package_manager_script_invocation")
    require_text("packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/syncpack_prettier_pin.rs", "first_matching_group_pins_dependency")
    require_text(roots_rs, "package_roots")
    require_text(roots_rs, "guardrail_roots")
    require_text(roots_rs, "intersection")
    forbid_text(fixture_package, '"format":')
    require_text(fixture_package, '"format:check": "prettier --check --cache ."')
    require_text(fixture_package, '"validate": "pnpm format:check"')
    require_file(fixture_guardrail)
    require_text(targetless_package, '"format:check": "prettier --check --cache-strategy metadata"')
    require_text(bare_targetless_package, '"format:check": "prettier --check"')
    require_text(or_package, "|| true")
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R00-clean-golden/repo/.syncpackrc", '"packages": ["**"]')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R00-clean-golden/repo/.syncpackrc", '"dependencyTypes": ["**"]')
    require_text(targetless_package, '"format:check": "prettier --check --cache-strategy metadata"')
    require_text(stdin_filepath_package, '"format:check": "prettier --check --stdin-filepath src/foo.ts"')
    require_text(find_config_path_package, '"format:check": "prettier --check --find-config-path src/foo.ts"')
    require_text(ignored_first_match_syncpack, '"isIgnored": true')
    require_text(banned_first_match_syncpack, '"isBanned":true')
    require_text(unpinned_first_match_syncpack, '"dependencies":["prettier"]')
    require_text(dependency_negation_syncpack, '"!prettier"')
    require_text(dependency_type_negation_syncpack, '"!dev"')
    require_text(specifier_type_negation_syncpack, '"!exact"')
    require_text(latest_specifier_syncpack_package, '"prettier": "latest"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/tag-specifier-syncpack-miss/package.json", '"prettier": "beta"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/file-specifier-syncpack-miss/package.json", '"prettier": "file:../prettier.tgz"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/link-specifier-syncpack-miss/package.json", '"prettier": "link:../prettier"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/git-specifier-syncpack-miss/package.json", '"prettier": "github:prettier/prettier"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/url-specifier-syncpack-miss/package.json", '"prettier": "https://example.com/prettier.tgz"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/catalog-specifier-syncpack-miss/package.json", '"prettier": "catalog:default"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/alias-specifier-syncpack-miss/package.json", '"prettier": "npm:prettier@3.6.2"')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/workspace-specifier-syncpack-miss/package.json", '"prettier": "workspace:^"')
    require_text(scoped_syncpack, '"packages": ["!fmt-scoped-syncpack-miss"]')
    require_text(specifier_syncpack, '"specifierTypes": ["exact"]')
    require_text("behavior/fixtures/g3ts-rule/fmt/fmt-R20-policy-violations/repo/specifier-syncpack-miss/package.json", '"prettier": "^3.6.2"')
    require_text(prod_syncpack, '"dependencyTypes": ["prod"]')
    require_text(prod_syncpack_applies, '"dependencies"')
    require_text(prod_syncpack_applies, '"prettier": "^3.6.2"')
    require_text(prod_syncpack_applies_config, '"specifierTypes": ["range"]')
    require_text(runner_direct_package, '"format:check": "pnpm exec prettier --check ."')
    require_text(filtered_shortcut_package, '"validate": "pnpm --filter other format:check"')
    require_text(indirect_script_package, '"validate": "pnpm run indirect"')
    require_text(hook_contract, '"**/guardrail3-ts.toml"')
    require_text(hook_contract, '"**/package.json"')
    require_text(hook_contract, '"**/.syncpackrc"')
    require_text(fmt_roots, 'normalized.starts_with(".prettierrc.")')
    require_text(fmt_roots, 'normalized.starts_with("prettier.config.")')


def verify_spelling() -> None:
    common_rs = "packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/common.rs"
    roots_rs = "packages/ts/spelling/g3ts-spelling-ingestion/crates/runtime/src/roots.rs"
    fixture = "behavior/fixtures/g3ts-rule/spelling/spelling-R40-bare-cspell/fixture.toml"
    fixture_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R40-bare-cspell/repo/package.json"
    fixture_guardrail = "behavior/fixtures/g3ts-rule/spelling/spelling-R00-clean-golden/repo/guardrail3-ts.toml"
    invalid_config = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/invalid-config/cspell.json"
    or_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/or-fallback/package.json"
    validate_bare_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/validate-bare/package.json"
    targetless_option_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/targetless-option/package.json"
    lint_targetless_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/lint-targetless/package.json"
    no_exit_code_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/no-exit-code/package.json"
    trace_subcommand_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/trace-subcommand/package.json"
    ignored_first_match_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/ignored-first-match-syncpack/.syncpackrc"
    banned_first_match_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/banned-first-match-syncpack/.syncpackrc"
    unpinned_first_match_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/unpinned-first-match-syncpack/.syncpackrc"
    latest_specifier_syncpack_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/latest-specifier-syncpack-miss/package.json"
    scoped_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/scoped-syncpack-miss/.syncpackrc"
    specifier_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/specifier-syncpack-miss/.syncpackrc"
    prod_syncpack = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/prod-syncpack-miss/.syncpackrc"
    prod_syncpack_applies = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/prod-dependency-syncpack-applies/package.json"
    prod_syncpack_applies_config = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/prod-dependency-syncpack-applies/.syncpackrc"
    runner_direct_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/runner-direct/package.json"
    filtered_shortcut_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R30-script-not-wired/repo/filtered-shortcut/package.json"
    indirect_script_package = "behavior/fixtures/g3ts-rule/spelling/spelling-R30-script-not-wired/repo/indirect-script/package.json"
    hook_contract = "packages/ts/spelling/g3ts-spelling-hook-contract/src/contract.rs"
    spelling_roots = "packages/ts/spelling/g3ts-spelling-ingestion/crates/runtime/src/roots.rs"

    require_text(common_rs, "cspell_args_have_target")
    require_text(common_rs, "cspell_option_takes_value")
    require_text(common_rs, "original_command_starts_with(invocation, \"cspell\")")
    require_text(common_rs, "invocation_uses_package_manager_script_invocation")
    require_text(common_rs, "cspell_args_disable_exit_code")
    require_text(common_rs, "cspell_non_lint_subcommand")
    require_text("packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/syncpack_cspell_pin.rs", "first_matching_group_pins_dependency")
    require_text(roots_rs, "package_roots")
    require_text(roots_rs, "guardrail_roots")
    require_text(roots_rs, "intersection")
    require_file(fixture)
    require_text(fixture_package, '"spellcheck": "cspell"')
    require_file(fixture_guardrail)
    require_text("behavior/fixtures/g3ts-rule/spelling/spelling-R00-clean-golden/repo/package.json", '"validate": "pnpm spellcheck"')
    require_text(invalid_config, "{ bad json")
    require_text(or_package, "|| true")
    require_text(validate_bare_package, '"validate": "cspell"')
    require_text(targetless_option_package, '"spellcheck": "cspell --root ."')
    require_text(lint_targetless_package, '"spellcheck": "cspell lint --root ."')
    require_text(no_exit_code_package, '"spellcheck": "cspell . --no-exit-code"')
    require_text(trace_subcommand_package, '"spellcheck": "cspell trace word"')
    require_text("behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/check-subcommand/package.json", '"spellcheck": "cspell check words"')
    require_text(ignored_first_match_syncpack, '"isIgnored": true')
    require_text(banned_first_match_syncpack, '"isBanned":true')
    require_text(unpinned_first_match_syncpack, '"dependencies":["cspell"]')
    require_text(latest_specifier_syncpack_package, '"cspell": "latest"')
    require_text("behavior/fixtures/g3ts-rule/spelling/spelling-R00-clean-golden/repo/.syncpackrc", '"packages": ["**"]')
    require_text("behavior/fixtures/g3ts-rule/spelling/spelling-R00-clean-golden/repo/.syncpackrc", '"dependencyTypes": ["**"]')
    require_text(scoped_syncpack, '"packages": ["!spelling-scoped-syncpack-miss"]')
    require_text(specifier_syncpack, '"specifierTypes": ["exact"]')
    require_text("behavior/fixtures/g3ts-rule/spelling/spelling-R20-policy-violations/repo/specifier-syncpack-miss/package.json", '"cspell": "^9.2.1"')
    require_text(prod_syncpack, '"dependencyTypes": ["prod"]')
    require_text(prod_syncpack_applies, '"dependencies"')
    require_text(prod_syncpack_applies, '"cspell": "^9.2.1"')
    require_text(prod_syncpack_applies_config, '"specifierTypes": ["range"]')
    require_text(runner_direct_package, '"spellcheck": "pnpm exec cspell ."')
    require_text(filtered_shortcut_package, '"validate": "pnpm --filter other spellcheck"')
    require_text(indirect_script_package, '"validate": "pnpm run indirect"')
    require_text(hook_contract, '"**/guardrail3-ts.toml"')
    require_text(hook_contract, '"**/package.json"')
    require_text(hook_contract, '"**/.syncpackrc"')
    require_text(hook_contract, '"cspell.config.*"')
    require_text(hook_contract, '"**/cspell.config.*"')
    require_text(spelling_roots, 'normalized.starts_with("cspell.config.")')


def verify_typecov() -> None:
    parser_types = "packages/parsers/g3ts-toml-parser/crates/types/src/guardrail3_ts_toml.rs"
    typecov_types = "packages/ts/typecov/g3ts-typecov-types/src/types.rs"
    ingestion_run = "packages/ts/typecov/g3ts-typecov-ingestion/crates/runtime/src/run.rs"
    common_rs = "packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/common.rs"
    hook_contract = "packages/ts/typecov/g3ts-typecov-hook-contract/src/contract.rs"
    roots_rs = "packages/ts/typecov/g3ts-typecov-ingestion/crates/runtime/src/roots.rs"
    policy_rs = "packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/policy_configured.rs"
    clean_config = "behavior/fixtures/g3ts-rule/typecov/typecov-R00-clean-golden/repo/guardrail3-ts.toml"
    non100_config = "behavior/fixtures/g3ts-rule/typecov/typecov-R50-non100-policy-clean/repo/guardrail3-ts.toml"
    duplicate_threshold_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/duplicate-threshold/package.json"
    invalid_threshold_value_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/invalid-threshold-value/package.json"
    threshold_above_100_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/threshold-above-100/package.json"
    attached_threshold_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/attached-threshold-below-policy/package.json"
    bare_script_shortcut_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R30-script-not-wired/repo/bare-script-shortcut/package.json"
    scoped_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/scoped-syncpack-miss/.syncpackrc"
    ignored_first_match_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/ignored-first-match-syncpack/.syncpackrc"
    banned_first_match_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/banned-first-match-syncpack/.syncpackrc"
    unpinned_first_match_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/unpinned-first-match-syncpack/.syncpackrc"
    latest_specifier_syncpack_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/latest-specifier-syncpack-miss/package.json"
    specifier_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/specifier-syncpack-miss/.syncpackrc"
    prod_syncpack = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/prod-syncpack-miss/.syncpackrc"
    prod_syncpack_applies = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/prod-dependency-syncpack-applies/package.json"
    prod_syncpack_applies_config = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/prod-dependency-syncpack-applies/.syncpackrc"
    runner_direct_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/runner-direct/package.json"
    filtered_shortcut_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R30-script-not-wired/repo/filtered-shortcut/package.json"
    indirect_script_package = "behavior/fixtures/g3ts-rule/typecov/typecov-R30-script-not-wired/repo/indirect-script/package.json"
    invalid_fixture = "behavior/fixtures/g3ts-rule/typecov/typecov-R40-policy-threshold/fixture.toml"

    require_text(parser_types, "typecov")
    require_text(parser_types, "pub minimum: Option<Value>")
    forbid_text(parser_types, "deserialize_typecov_minimum")
    require_text(typecov_types, "G3TsTypecovPolicy")
    require_text(ingestion_run, "typecov_policy")
    require_text(common_rs, "--strict")
    require_text(common_rs, "--at-least")
    require_text(common_rs, "minimum")
    require_text(common_rs, "type_coverage_thresholds")
    require_text(common_rs, "*threshold <= 100")
    require_text(common_rs, "strip_prefix(\"--at-least=\")")
    require_text(common_rs, "original_command_starts_with(invocation, \"type-coverage\")")
    require_text(common_rs, "invocation_uses_package_manager_script_invocation")
    require_text("packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/syncpack_type_coverage_pin.rs", "first_matching_group_pins_dependency")
    require_text(hook_contract, "guardrail3-ts.toml")
    require_text(hook_contract, '"**/guardrail3-ts.toml"')
    require_text(hook_contract, '"**/package.json"')
    require_text(hook_contract, '"**/.syncpackrc"')
    require_text(roots_rs, "package_roots")
    require_text(roots_rs, "guardrail_roots")
    require_text(roots_rs, "intersection")
    require_text(policy_rs, "0..=100")
    forbid_text(common_rs, "type_coverage_invocation_at_100")
    require_text(clean_config, "[typecov]")
    require_text(clean_config, "minimum = 100")
    require_text(non100_config, "minimum = 90")
    require_text(duplicate_threshold_package, "--at-least 100 --at-least 0")
    require_text(invalid_threshold_value_package, "--at-least 100 --at-least nope")
    require_text(threshold_above_100_package, "--at-least 255")
    require_text(attached_threshold_package, "--at-least=0 --at-least 100")
    require_text(bare_script_shortcut_package, '"validate": "typecov"')
    require_text(ignored_first_match_syncpack, '"isIgnored": true')
    require_text(banned_first_match_syncpack, '"isBanned":true')
    require_text(unpinned_first_match_syncpack, '"dependencies":["type-coverage"]')
    require_text(latest_specifier_syncpack_package, '"type-coverage": "latest"')
    require_text("behavior/fixtures/g3ts-rule/typecov/typecov-R00-clean-golden/repo/.syncpackrc", '"packages": ["**"]')
    require_text("behavior/fixtures/g3ts-rule/typecov/typecov-R00-clean-golden/repo/.syncpackrc", '"dependencyTypes": ["**"]')
    require_text(scoped_syncpack, '"packages": ["!typecov-scoped-syncpack-miss"]')
    require_text(specifier_syncpack, '"specifierTypes": ["exact"]')
    require_text("behavior/fixtures/g3ts-rule/typecov/typecov-R20-policy-violations/repo/specifier-syncpack-miss/package.json", '"type-coverage": "^2.29.7"')
    require_text(prod_syncpack, '"dependencyTypes": ["prod"]')
    require_text(prod_syncpack_applies, '"dependencies"')
    require_text(prod_syncpack_applies, '"type-coverage": "^2.29.7"')
    require_text(prod_syncpack_applies_config, '"specifierTypes": ["range"]')
    require_text(runner_direct_package, '"typecov": "pnpm exec type-coverage --strict --at-least 90"')
    require_text(filtered_shortcut_package, '"validate": "pnpm --filter other typecov"')
    require_text(indirect_script_package, '"validate": "pnpm run indirect"')
    require_text("behavior/fixtures/g3ts-rule/typecov/typecov-R50-non100-policy-clean/repo/package.json", '"validate": "type-coverage --strict --at-least 95"')
    require_file(invalid_fixture)


def main() -> int:
    verify_manifest_loads()
    verify_generated_hook()
    verify_shared_parser_ownership()
    verify_fmt()
    verify_spelling()
    verify_typecov()
    print("g3ts-tool-family-hardening: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
