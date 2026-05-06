use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookRequirement, G3HookTriggerPattern,
};
use g3rs_hooks_source_checks_assertions::run as assertions;
use g3rs_hooks_source_checks_assertions::run::ExpectedRuleResult;
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use hook_shell_parser::parse_script;

use super::super::check;
use super::super::check_all;

const VALID_PRECOMMIT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(git rev-parse --show-toplevel)"
"$REPO_ROOT/scripts/g3rs/verify" --mode pre-commit --scope "$REPO_ROOT/apps/guardrail3-rs"
"#;

const VALID_VERIFIER: &str = r#"#!/usr/bin/env bash
set -euo pipefail
usage() { exit 2; }
[[ -n "$SCOPE_ARG" ]] || usage 'missing --scope'
case "$MODE" in
  pre-commit)
    ;;
  workspace)
    ;;
  *)
    usage "unknown --mode: $MODE"
    ;;
esac
export CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"
g3rs validate --path "$SCOPE"
cargo metadata --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo machete
cargo test --workspace
cargo mutants --check --in-place
cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests
"#;

#[test]
fn hook_passes_when_it_calls_g3rs_verifier_with_mode_and_scope() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        VALID_PRECOMMIT,
        Vec::new(),
    ));

    assert_inventory(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "pre-commit calls Rust verifier",
    );
}

#[test]
fn hook_fails_when_it_does_not_call_g3rs_verifier() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\ncargo test --workspace\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        ".githooks/pre-commit must run scripts/g3rs/verify",
    );
}

#[test]
fn hook_fails_when_g3rs_verifier_line_omits_precommit_mode() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\n\"$REPO_ROOT/scripts/g3rs/verify\" --scope apps/guardrail3-rs\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "--mode pre-commit",
    );
}

#[test]
fn hook_fails_when_g3rs_verifier_line_omits_scope() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\n\"$REPO_ROOT/scripts/g3rs/verify\" --mode pre-commit\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "--scope",
    );
}

#[test]
fn verifier_facts_pass_with_required_commands_and_no_g3ts_script() {
    let results = check_all(&[
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            VALID_PRECOMMIT,
            Vec::new(),
        ),
        input(
            "scripts/g3rs/verify",
            G3RsHookScriptKind::G3RsVerifier,
            VALID_VERIFIER,
            Vec::new(),
        ),
    ]);

    assert_inventory(
        &results,
        "g3rs-hooks/g3rs-verifier-exists",
        "Rust verifier script exists",
    );
    assert_no_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-forbidden-tools",
        "must not call g3ts",
    );
}

#[test]
fn verifier_fails_when_missing_g3rs_validate_scope() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("g3rs validate --path \"$SCOPE\"\n", ""),
        "g3rs validate --path \"$SCOPE\"",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_metadata_locked() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo metadata --locked\n", ""),
        "cargo metadata --locked",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_fmt_all_check() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo fmt --all -- --check\n", ""),
        "cargo fmt --all -- --check",
    );
}

#[test]
fn verifier_fails_when_clippy_omits_warning_denial() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" -- -D warnings", ""),
        "cargo clippy --workspace --all-targets --all-features -- -D warnings",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_deny_check() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo deny check\n", ""),
        "cargo deny check",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_machete() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo machete\n", ""),
        "cargo machete",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_test_workspace() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo test --workspace\n", ""),
        "cargo test --workspace",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_mutants_check_in_place() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo mutants --check --in-place\n", ""),
        "cargo mutants --check --in-place",
    );
}

#[test]
fn verifier_fails_when_cargo_dupes_omits_thresholds() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" --max-exact 85 --max-exact-percent 10", ""),
        "cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests",
    );
}

#[test]
fn verifier_fails_when_cargo_dupes_omits_exclude_tests() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" --exclude-tests", ""),
        "cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests",
    );
}

#[test]
fn verifier_fails_when_it_calls_g3ts() {
    let results = check(&input(
        "scripts/g3rs/verify",
        G3RsHookScriptKind::G3RsVerifier,
        &format!("{VALID_VERIFIER}g3ts validate --path \"$SCOPE\"\n"),
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-forbidden-tools",
        "must not call g3ts",
    );
}

#[test]
fn verifier_fails_when_it_calls_typescript_package_managers() {
    for package_manager in ["pnpm", "npm", "yarn", "bun"] {
        let results = check(&input(
            "scripts/g3rs/verify",
            G3RsHookScriptKind::G3RsVerifier,
            &format!("{VALID_VERIFIER}{package_manager} install\n"),
            Vec::new(),
        ));

        assert_finding(
            &results,
            "g3rs-hooks/g3rs-verifier-forbidden-tools",
            "must not call pnpm, npm, yarn, or bun",
        );
    }
}

#[test]
fn required_contract_commands_are_checked_across_modular_hook_surface() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\nrun-parts .githooks/pre-commit.d\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(true),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn orphan_modular_hook_script_does_not_satisfy_pre_commit_contract_surface() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\necho no dispatcher\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(false),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn sourcing_one_modular_script_does_not_dispatch_entire_directory() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\nsource .githooks/pre-commit.d/bootstrap\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(false),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

fn input(
    rel_path: &str,
    kind: G3RsHookScriptKind,
    content: &str,
    requirements: Vec<G3HookRequirement>,
) -> G3RsHooksSourceChecksInput {
    G3RsHooksSourceChecksInput {
        rel_path: rel_path.to_owned(),
        kind,
        exists: true,
        parsed: parse_script(content),
        has_modular_dir: true,
        is_workspace_project: true,
        requirements,
    }
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::Glob("**/*.rs".to_owned())],
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}

fn assert_verifier_missing(content: &str, expected_message: &str) {
    let results = check(&input(
        "scripts/g3rs/verify",
        G3RsHookScriptKind::G3RsVerifier,
        content,
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-required-commands",
        expected_message,
    );
}

fn assert_finding(results: &[guardrail3_check_types::G3CheckResult], id: &str, message: &str) {
    assert!(
        results.iter().any(|result| result.id() == id
            && !result.inventory()
            && result.message().contains(message)),
        "expected non-inventory result {id} containing {message}; got {results:#?}",
    );
}

fn assert_inventory(results: &[guardrail3_check_types::G3CheckResult], id: &str, title: &str) {
    assert!(
        results.iter().any(|result| result.id() == id
            && result.inventory()
            && result.title().contains(title)),
        "expected inventory result {id} containing {title}; got {results:#?}",
    );
}

fn assert_no_finding(results: &[guardrail3_check_types::G3CheckResult], id: &str, message: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id
            && !result.inventory()
            && result.message().contains(message)),
        "expected no non-inventory result {id} containing {message}; got {results:#?}",
    );
}
