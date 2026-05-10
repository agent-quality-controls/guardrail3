use g3ts_hooks_source_checks_assertions as assertions;
use g3ts_hooks_types::{
    G3TsHookScriptKind, G3TsHooksEnabledCategories, G3TsHooksSourceChecksInput,
};
use hook_shell_parser::parse_script;

use super::super::check;

const REAL_PRE_COMMIT_HOOK: &str = include_str!("../../../../../../../../.githooks/pre-commit");

fn real_hook_input(content: &str) -> G3TsHooksSourceChecksInput {
    G3TsHooksSourceChecksInput::new(
        ".githooks/pre-commit".to_owned(),
        G3TsHookScriptKind::PreCommit,
        parse_script(content),
        false,
        Vec::new(),
        G3TsHooksEnabledCategories::all(),
        Vec::new(),
    )
}

fn synthetic_hook_input(content: &str) -> G3TsHooksSourceChecksInput {
    G3TsHooksSourceChecksInput::new(
        ".githooks/pre-commit".to_owned(),
        G3TsHookScriptKind::PreCommit,
        parse_script(content),
        false,
        vec!["apps/landing".to_owned()],
        G3TsHooksEnabledCategories::all(),
        Vec::new(),
    )
}

#[test]
fn real_repo_pre_commit_hook_passes_all_rules() {
    let results = check(&real_hook_input(REAL_PRE_COMMIT_HOOK));
    assertions::run::assert_no_non_inventory_findings(&results, "real .githooks/pre-commit");
}

#[test]
fn fires_calls_validate_repo_when_validate_repo_is_stripped() {
    let broken = REAL_PRE_COMMIT_HOOK.replace("g3ts validate-repo\n", "");
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "stripping `g3ts validate-repo` must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::dispatch::calls_validate_repo::rule::assert_error_finding(&results);
}

#[test]
fn fires_dispatches_per_unit_when_per_unit_validate_stripped() {
    let broken =
        REAL_PRE_COMMIT_HOOK.replace("        g3ts validate --path \"$unit\" --staged\n", "");
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "stripping per-unit `g3ts validate --path --staged` must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::dispatch::dispatches_per_unit_validate_staged::rule::assert_error_finding(&results);
}

#[test]
fn fires_dedups_when_dedup_stripped() {
    let broken = REAL_PRE_COMMIT_HOOK
        .replace(" | awk 'NF' | sort -u", "")
        .replace("awk '!seen", "echo '!seen");
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "stripping dedup must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::dispatch::dedups_owning_units::rule::assert_error_finding(&results);
}

#[test]
fn fires_skips_when_no_owning_unit_for_synthetic_hook_without_skip_guards() {
    let hook = r#"#!/usr/bin/env bash
set -e
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)
gitleaks protect --staged --no-banner
if grep -E '<<<<<<<|=======|>>>>>>>' file; then exit 1; fi
file_size=$(git cat-file -s HEAD)
echo drizzle/
echo db/schema/
pnpm install --frozen-lockfile
echo package.json
g3ts validate-repo
TS_UNIQUE_UNITS=$(printf '%s' "$STAGED_FILES" | awk 'NF' | sort -u)
for unit in $TS_UNIQUE_UNITS; do
    g3ts validate --path "$unit" --staged
done
"#;
    let results = check(&synthetic_hook_input(hook));
    assertions::dispatch::skips_when_no_owning_unit::rule::assert_error_finding(&results);
}

#[test]
fn fires_no_toolchain_invocation_when_pnpm_lint_is_injected() {
    let broken = REAL_PRE_COMMIT_HOOK.replace(
        "echo \"All pre-commit checks passed.\"",
        "pnpm exec eslint --max-warnings 0 .\necho \"All pre-commit checks passed.\"",
    );
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "injecting `pnpm exec eslint` must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::dispatch::no_toolchain_invocation::rule::assert_error_finding(&results);
}

#[test]
fn fires_no_toolchain_invocation_for_each_forbidden_command() {
    for cmd in [
        "tsc --noEmit",
        "eslint .",
        "prettier --check .",
        "cspell .",
        "stylelint **/*.css",
    ] {
        let broken = REAL_PRE_COMMIT_HOOK.replace(
            "echo \"All pre-commit checks passed.\"",
            &format!("{cmd}\necho \"All pre-commit checks passed.\""),
        );
        assert_ne!(
            broken, REAL_PRE_COMMIT_HOOK,
            "injecting `{cmd}` must alter the hook content",
        );
        let results = check(&real_hook_input(&broken));
        assertions::dispatch::no_toolchain_invocation::rule::assert_error_finding(&results);
    }
}

#[test]
fn fires_gitleaks_scan_when_gitleaks_stripped() {
    let broken = REAL_PRE_COMMIT_HOOK.replace(
        "if ! gitleaks protect --staged --no-banner; then",
        "if ! true; then",
    );
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "stripping gitleaks must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::scan::gitleaks_scan::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_discovers_marker_pair_when_one_marker_stripped() {
    let broken = REAL_PRE_COMMIT_HOOK.replace("guardrail3-ts.toml", "package.json");
    assert_ne!(
        broken, REAL_PRE_COMMIT_HOOK,
        "stripping the second TS marker must alter the hook content",
    );
    let results = check(&real_hook_input(&broken));
    assertions::routing::discovers_marker_pair::rule::assert_error_finding(&results);
}

#[test]
fn fires_merge_conflict_scan_when_marker_block_removed() {
    let lines: Vec<&str> = REAL_PRE_COMMIT_HOOK.lines().collect();
    let mut buf = String::new();
    let mut skipping = false;
    for line in lines {
        if !skipping && line.contains("Checking for merge conflict markers") {
            skipping = true;
            continue;
        }
        if skipping && line.trim_start() == "fi" {
            skipping = false;
            continue;
        }
        if skipping {
            continue;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    let results = check(&real_hook_input(&buf));
    assertions::scan::merge_conflict_scan::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_scope_not_hardcoded_literal_when_unit_is_replaced_by_literal() {
    let broken = REAL_PRE_COMMIT_HOOK.replacen(
        "g3ts validate --path \"$unit\" --staged",
        "g3ts validate --path \"apps/landing\" --staged",
        1,
    );
    assert_ne!(broken, REAL_PRE_COMMIT_HOOK);
    let results = check(&real_hook_input(&broken));
    assertions::routing::scope_not_hardcoded_literal::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_scope_not_hardcoded_literal_for_repo_root_prefixed_literal() {
    let broken = REAL_PRE_COMMIT_HOOK.replacen(
        "g3ts validate --path \"$unit\" --staged",
        "g3ts validate --path \"$REPO_ROOT/apps/landing\" --staged",
        1,
    );
    assert_ne!(broken, REAL_PRE_COMMIT_HOOK);
    let results = check(&real_hook_input(&broken));
    assertions::routing::scope_not_hardcoded_literal::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_scope_for_ambient_repo_root_after_loop() {
    let broken = REAL_PRE_COMMIT_HOOK.replace(
        "echo \"All pre-commit checks passed.\"",
        "g3ts validate --path \"$REPO_ROOT\" --staged\necho \"All pre-commit checks passed.\"",
    );
    let results = check(&real_hook_input(&broken));
    assertions::routing::scope_not_hardcoded_literal::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_no_env_override_for_default_substitution() {
    let injected = format!(
        "TS_PKG=\"${{GUARDRAIL3_TS_SCOPE:-apps/landing}}\"\n{}",
        REAL_PRE_COMMIT_HOOK.replacen(
            "g3ts validate --path \"$unit\" --staged",
            "g3ts validate --path \"$TS_PKG\" --staged",
            1,
        )
    );
    let results = check(&real_hook_input(&injected));
    assertions::routing::no_env_override::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_no_env_override_for_command_substitution_default() {
    let injected = format!(
        "TS_PKG=\"$(cat /etc/foo 2>/dev/null || echo apps/landing)\"\n{}",
        REAL_PRE_COMMIT_HOOK.replacen(
            "g3ts validate --path \"$unit\" --staged",
            "g3ts validate --path \"$TS_PKG\" --staged",
            1,
        )
    );
    let results = check(&real_hook_input(&injected));
    assertions::routing::no_env_override::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_no_env_override_for_default_fallback_literal() {
    let injected = REAL_PRE_COMMIT_HOOK.replace(
        "if [ -n \"$TS_UNIQUE_UNITS\" ]",
        "if [ -z \"$TS_UNIQUE_UNITS\" ]; then TS_UNIQUE_UNITS=\"apps/landing\"; fi\nif [ -n \"$TS_UNIQUE_UNITS\" ]",
    );
    let results = check(&real_hook_input(&injected));
    assertions::routing::no_env_override::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_no_env_override_for_default_fallback_variable_prefixed_literal() {
    let injected = REAL_PRE_COMMIT_HOOK.replace(
        "if [ -n \"$TS_UNIQUE_UNITS\" ]",
        "if [ -z \"$TS_UNIQUE_UNITS\" ]; then TS_UNIQUE_UNITS=\"$REPO_ROOT/apps/landing\"; fi\nif [ -n \"$TS_UNIQUE_UNITS\" ]",
    );
    let results = check(&real_hook_input(&injected));
    assertions::routing::no_env_override::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_staged_files_diff_filter_acm_when_filter_omitted() {
    let broken = REAL_PRE_COMMIT_HOOK.replace(
        "git diff --cached --name-only --diff-filter=ACM",
        "git diff --cached --name-only",
    );
    assert_ne!(broken, REAL_PRE_COMMIT_HOOK);
    let results = check(&real_hook_input(&broken));
    assertions::routing::staged_files_diff_filter_acm::rule::assert_error_finding(&results);
}

#[test]
fn fires_file_size_cap_when_size_check_stripped() {
    let stripped: String = REAL_PRE_COMMIT_HOOK
        .lines()
        .filter(|line| {
            !line.contains("MAX_FILE_SIZE")
                && !line.contains("file_size")
                && !line.contains("git cat-file -s")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let results = check(&real_hook_input(&stripped));
    assertions::scan::file_size_cap::rule::assert_error_finding(&results);
}

#[test]
fn fires_migration_consistency_when_migration_block_stripped() {
    let stripped: String = REAL_PRE_COMMIT_HOOK
        .lines()
        .filter(|line| !line.contains("drizzle/") && !line.contains("db/schema/"))
        .collect::<Vec<_>>()
        .join("\n");
    let results = check(&real_hook_input(&stripped));
    assertions::consistency::migration_consistency::rule::assert_error_finding(&results);
}

#[test]
fn fires_lockfile_integrity_when_block_stripped() {
    let stripped: String = REAL_PRE_COMMIT_HOOK
        .lines()
        .filter(|line| {
            !line.contains("pnpm install --frozen-lockfile")
                && !line.contains("npm install --package-lock-only")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let results = check(&real_hook_input(&stripped));
    assertions::consistency::lockfile_integrity::rule::assert_error_finding(&results);
}

#[test]
fn fires_routing_no_upward_walk_when_dirname_assigns_parent_unit() {
    let broken = REAL_PRE_COMMIT_HOOK.replace(
        "echo \"All pre-commit checks passed.\"",
        "for unit in $TS_UNIQUE_UNITS; do parent_unit=$(dirname \"$unit\"); g3ts validate --path \"$parent_unit\" --staged; done\necho \"All pre-commit checks passed.\"",
    );
    let results = check(&real_hook_input(&broken));
    assertions::routing::no_upward_walk_from_units::rule::assert_error_finding(&results);
}

#[test]
fn synthetic_minimal_hook_passes_all_rules() {
    let hook = r#"#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(git rev-parse --show-toplevel)"
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)
if grep -qE '^(<<<<<<<|=======|>>>>>>>)' file; then exit 1; fi
gitleaks protect --staged --no-banner
MAX_FILE_SIZE=1048576
file_size=$(git cat-file -s HEAD)
echo drizzle/foo.sql
echo db/schema/foo.ts
pnpm install --frozen-lockfile
echo package.json
g3ts validate-repo
TS_UNIQUE_UNITS=$(printf '%s' "$STAGED_FILES" | awk 'NF' | sort -u)
[ -f "$REPO_ROOT/apps/landing/package.json" ] && [ -f "$REPO_ROOT/apps/landing/guardrail3-ts.toml" ] && true
while IFS= read -r unit; do
    [ -n "$unit" ] || continue
    g3ts validate --path "$unit" --staged
done <<< "$TS_UNIQUE_UNITS"
"#;
    let results = check(&synthetic_hook_input(hook));
    assertions::run::assert_no_non_inventory_findings(&results, "synthetic minimal hook");
    assertions::dispatch::calls_validate_repo::rule::assert_rule_quiet(&results);
    assertions::dispatch::dispatches_per_unit_validate_staged::rule::assert_rule_quiet(&results);
    assertions::dispatch::dedups_owning_units::rule::assert_rule_quiet(&results);
    assertions::dispatch::skips_when_no_owning_unit::rule::assert_rule_quiet(&results);
    assertions::dispatch::no_toolchain_invocation::rule::assert_rule_quiet(&results);
    assertions::scan::gitleaks_scan::rule::assert_rule_quiet(&results);
    assertions::scan::merge_conflict_scan::rule::assert_rule_quiet(&results);
    assertions::scan::file_size_cap::rule::assert_rule_quiet(&results);
    assertions::routing::discovers_marker_pair::rule::assert_rule_quiet(&results);
    assertions::routing::scope_not_hardcoded_literal::rule::assert_rule_quiet(&results);
    assertions::routing::no_env_override::rule::assert_rule_quiet(&results);
    assertions::routing::staged_files_diff_filter_acm::rule::assert_rule_quiet(&results);
    assertions::routing::no_upward_walk_from_units::rule::assert_rule_quiet(&results);
    assertions::consistency::migration_consistency::rule::assert_rule_quiet(&results);
    assertions::consistency::lockfile_integrity::rule::assert_rule_quiet(&results);
}
