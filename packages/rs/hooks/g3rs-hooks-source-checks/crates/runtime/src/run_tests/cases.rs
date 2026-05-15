use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookRequirement, G3HookTriggerPattern,
};
use g3rs_hooks_source_checks_assertions as assertions;
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use hook_shell_parser::parse_script;

use super::super::check;

const REAL_MANAGED_G3RS_HOOK: &str =
    include_str!("../../../../../../../../.githooks/pre-commit.d/g3rs");

fn real_managed_hook_input(content: &str) -> G3RsHooksSourceChecksInput {
    G3RsHooksSourceChecksInput {
        rel_path: ".githooks/pre-commit.d/g3rs".to_owned(),
        kind: G3RsHookScriptKind::Modular,
        exists: true,
        parsed: parse_script(content),
        has_modular_dir: true,
        is_workspace_project: true,
        requirements: Vec::new(),
    }
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
        has_modular_dir: false,
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

#[test]
fn real_repo_pre_commit_hook_passes_all_rules() {
    let results = check(&real_managed_hook_input(REAL_MANAGED_G3RS_HOOK));
    assertions::run::assert_no_non_inventory_findings(&results, "real .githooks/pre-commit.d/g3rs");
}

#[test]
fn real_repo_pre_commit_hook_with_validate_repo_stripped_fires_calls_validate_repo() {
    let broken = REAL_MANAGED_G3RS_HOOK.replace("g3rs validate repo --path \"$repo_root\"\n", "");
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "stripping `g3rs validate repo` must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::dispatch::calls_validate_repo::rule::assert_error_finding(&results);
}

#[test]
fn real_repo_pre_commit_hook_with_per_unit_validate_stripped_fires_dispatches() {
    let broken = REAL_MANAGED_G3RS_HOOK.replace(
        "        g3rs validate workspace --path \"$unit\" --staged\n",
        "",
    );
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "stripping per-unit `g3rs validate workspace --path --staged` must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::dispatch::dispatches_per_unit_validate_staged::rule::assert_error_finding(&results);
}

#[test]
fn real_repo_pre_commit_hook_with_dedup_stripped_fires_dedups() {
    // The hook uses `awk 'NF' | sort -u` to dedup discovered units; stripping `sort -u`
    // and the awk dedup makes the check fire.
    let broken = REAL_MANAGED_G3RS_HOOK
        .replace(" | awk 'NF' | sort -u", "")
        .replace("awk '!seen", "echo '!seen");
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "stripping dedup must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::dispatch::dedups_owning_units::rule::assert_error_finding(&results);
}

#[test]
fn skip_guards_missing_fires_skips_when_no_owning_unit() {
    // A synthetic hook that lacks any `[ -z ... ]`/`[ -n ... ]` test or `continue`
    // should fire `skips-when-no-owning-unit`.
    let hook = r#"#!/usr/bin/env bash
set -e
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)
gitleaks protect --staged --no-banner
g3rs validate repo
RUST_UNIQUE_UNITS=$(printf '%s' "$STAGED_FILES" | awk 'NF' | sort -u)
for unit in $RUST_UNIQUE_UNITS; do
    g3rs validate workspace --path "$unit" --staged
done
"#;
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        hook,
        Vec::new(),
    ));
    assertions::dispatch::skips_when_no_owning_unit::rule::assert_error_finding(&results);
}

#[test]
fn real_repo_pre_commit_hook_with_cargo_invocation_fires_no_toolchain_invocation() {
    let broken = format!("{REAL_MANAGED_G3RS_HOOK}\ncargo --version\n");
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "injecting `cargo` must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::dispatch::no_toolchain_invocation::rule::assert_error_finding(&results);
}

#[test]
fn real_repo_pre_commit_hook_with_gitleaks_stripped_fires_error() {
    let broken = REAL_MANAGED_G3RS_HOOK.replace(
        "if ! gitleaks protect --staged --no-banner; then",
        "if ! true; then",
    );
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "stripping gitleaks must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::gitleaks_step_present::rule::assert_error_finding(&results);
}

#[test]
fn real_repo_pre_commit_hook_with_marker_stripped_fires_error() {
    let broken = REAL_MANAGED_G3RS_HOOK.replace("guardrail3-rs.toml", "Cargo.toml");
    assert_ne!(
        broken, REAL_MANAGED_G3RS_HOOK,
        "stripping the second marker must alter the hook content",
    );
    let results = check(&real_managed_hook_input(&broken));
    assertions::routing::discovers_marker_pair::assert_error_finding(&results);
}

const HOOK_DELEGATING_VIA_VALIDATE_STAGED: &str = r#"#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(git rev-parse --show-toplevel)"
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)
gitleaks protect --staged --no-banner
g3rs validate repo
g3ts validate-repo
RUST_UNIQUE_UNITS=$(printf '%s' "$RUST_OWNING_UNITS" | awk 'NF' | sort -u)
while IFS= read -r unit; do
    [ -n "$unit" ] || continue
    g3rs validate workspace --path "$unit" --staged
done <<< "$RUST_UNIQUE_UNITS"
"#;

const HOOK_WITHOUT_DELEGATION: &str = r"#!/usr/bin/env bash
set -euo pipefail
echo nothing here
";

const FAMILY_DELEGATED_REQUIREMENTS: &[G3HookCommandRequirement] = &[
    G3HookCommandRequirement::CargoFmtCheck,
    G3HookCommandRequirement::CargoClippyDenyWarnings,
    G3HookCommandRequirement::CargoDenyCheck,
    G3HookCommandRequirement::ConcreteLockfileCommand,
    G3HookCommandRequirement::CargoTest,
    G3HookCommandRequirement::CargoMachete,
    G3HookCommandRequirement::CargoDupes,
    G3HookCommandRequirement::CargoDupesExcludeTests,
    G3HookCommandRequirement::G3RsValidatePath,
];

#[test]
fn family_owned_commands_emit_inventory_when_hook_dispatches_validate_staged() {
    let mut requirements = Vec::new();
    for command in FAMILY_DELEGATED_REQUIREMENTS {
        requirements.push(requirement(*command));
    }
    let hook_input = input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        HOOK_DELEGATING_VIA_VALIDATE_STAGED,
        requirements,
    );
    let results = check(&hook_input);
    assertions::required_contract_command_present::rule::assert_family_delegated_inventory(
        &results,
        "test",
        FAMILY_DELEGATED_REQUIREMENTS.len(),
    );
}

#[test]
fn family_owned_commands_warn_when_hook_neither_delegates_nor_runs_them() {
    let mut requirements = Vec::new();
    for command in FAMILY_DELEGATED_REQUIREMENTS {
        requirements.push(requirement(*command));
    }
    let hook_input = input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        HOOK_WITHOUT_DELEGATION,
        requirements,
    );
    let results = check(&hook_input);
    assertions::required_contract_command_present::rule::assert_family_missing_findings(
        &results,
        "test",
        FAMILY_DELEGATED_REQUIREMENTS.len(),
    );
}

#[test]
fn gitleaks_requirement_is_not_satisfied_by_validate_staged_delegation() {
    let hook_input = input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        HOOK_DELEGATING_VIA_VALIDATE_STAGED,
        vec![requirement(G3HookCommandRequirement::Gitleaks)],
    );
    let results = check(&hook_input);
    // Hook contains `gitleaks protect --staged --no-banner`, so the requirement is satisfied
    // by direct presence (inline), not by delegation.
    assertions::required_contract_command_present::rule::assert_family_inline_inventory(
        &results, "test",
    );
}

#[test]
fn passes_all_rules_for_inline_synthesised_hook() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        HOOK_DELEGATING_VIA_VALIDATE_STAGED,
        Vec::new(),
    ));
    assertions::dispatch::calls_validate_repo::rule::assert_inventory_only(&results);
    assertions::dispatch::dispatches_per_unit_validate_staged::rule::assert_inventory_only(
        &results,
    );
    assertions::dispatch::dedups_owning_units::rule::assert_inventory_only(&results);
    assertions::dispatch::skips_when_no_owning_unit::rule::assert_inventory_only(&results);
    assertions::dispatch::no_toolchain_invocation::rule::assert_inventory_only(&results);
}
