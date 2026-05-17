use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::any_resolved_command;

/// Result identifier for the `calls-validate-repo` rule.
const CALLS_VALIDATE_REPO_ID: &str = "g3ts-hooks/calls-validate-repo";
/// Result identifier for the per-unit dispatch rule.
const DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID: &str =
    "g3ts-hooks/dispatches-per-unit-validate-staged";
/// Result identifier for the dedup rule on discovered owning units.
const DEDUPS_OWNING_UNITS_ID: &str = "g3ts-hooks/dedups-owning-units";
/// Result identifier for the lenient-skip rule when no owning unit exists.
const SKIPS_WHEN_NO_OWNING_UNIT_ID: &str = "g3ts-hooks/skips-when-no-owning-unit";
/// Result identifier for the rule forbidding direct toolchain invocations.
const NO_TOOLCHAIN_INVOCATION_ID: &str = "g3ts-hooks/no-toolchain-invocation";
/// Result identifier for the merge-conflict marker scan rule.
const MERGE_CONFLICT_SCAN_ID: &str = "g3ts-hooks/merge-conflict-scan";
/// Result identifier for the gitleaks staged-files scan rule.
const GITLEAKS_SCAN_ID: &str = "g3ts-hooks/gitleaks-scan";
/// Result identifier for the staged-file size-cap rule.
const FILE_SIZE_CAP_ID: &str = "g3ts-hooks/file-size-cap";
/// Result identifier for the migration-consistency rule.
const MIGRATION_CONSISTENCY_ID: &str = "g3ts-hooks/migration-consistency";
/// Result identifier for the lockfile-integrity rule.
const LOCKFILE_INTEGRITY_ID: &str = "g3ts-hooks/lockfile-integrity";

#[must_use]
pub fn check(input: &G3TsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_effective(std::slice::from_ref(input))
}

#[must_use]
pub fn check_effective(inputs: &[G3TsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let Some(primary) = inputs
        .iter()
        .find(|input| input.kind() == G3TsHookScriptKind::PreCommit)
        .or_else(|| inputs.first())
    else {
        return results;
    };

    if primary.kind() != G3TsHookScriptKind::PreCommit {
        return results;
    }

    crate::routing::scope_not_hardcoded_literal::check(primary, &mut results);
    crate::routing::no_env_override_routing::check(primary, &mut results);
    crate::routing::no_upward_walk_from_units::check(primary, &mut results);
    crate::routing::discovers_marker_pair::check(primary, &mut results);
    crate::routing::staged_files_diff_filter_acm::check(primary, &mut results);

    check_calls_validate_repo(primary, &mut results);
    check_dispatches_per_unit_validate_staged(primary, &mut results);
    check_dedups_owning_units(primary, &mut results);
    check_skips_when_no_owning_unit(primary, &mut results);
    check_no_toolchain_invocation(primary, &mut results);

    check_merge_conflict_scan(primary, &mut results);
    check_gitleaks_scan(primary, &mut results);
    check_file_size_cap(primary, &mut results);
    check_migration_consistency(primary, &mut results);
    check_lockfile_integrity(primary, &mut results);

    results
}

/// Records a finding when the hook does not call `g3ts validate-repo`.
fn check_calls_validate_repo(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed(), |command| {
        command.command_name() == "g3ts"
            && command.args().first().map(String::as_str) == Some("validate-repo")
    });
    if found {
        return;
    }
    results.push(G3CheckResult::new(
        CALLS_VALIDATE_REPO_ID.to_owned(),
        G3Severity::Error,
        "hook does not call g3ts validate-repo".to_owned(),
        ".githooks/pre-commit must invoke `g3ts validate-repo` to enforce repo-level invariants \
         (hook shape, tool presence, repo-wide topology, marker-pair completeness)."
            .to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook does not invoke per-unit `g3ts validate --staged` from a loop.
fn check_dispatches_per_unit_validate_staged(
    input: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let lines: Vec<&str> = input
        .parsed()
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect();
    let mut depth: usize = 0;
    let mut found = false;
    for line in &lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth = depth.saturating_add(1);
            continue;
        }
        if trimmed.starts_with("done") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if depth > 0 && line_is_g3ts_validate_path_staged(trimmed) {
            found = true;
            break;
        }
    }
    if found {
        return;
    }
    results.push(G3CheckResult::new(
        DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID.to_owned(),
        G3Severity::Error,
        "hook does not dispatch per-unit g3ts validate --staged".to_owned(),
        ".githooks/pre-commit must invoke `g3ts validate --path <unit> --staged` from a discovery loop over staged files."
            .to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Returns true when `line` invokes `g3ts validate --path ... --staged`.
fn line_is_g3ts_validate_path_staged(line: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(line);
    for (idx, pair) in words.windows(2).enumerate() {
        let [first, second] = pair else { continue };
        if first == "g3ts" && second == "validate" {
            let tail_start = idx.saturating_add(2);
            let tail = words.get(tail_start..).unwrap_or(&[]);
            let has_staged = tail.iter().any(|w| w == "--staged");
            let has_path = tail
                .iter()
                .any(|w| w == "--path" || w.starts_with("--path="));
            return has_staged && has_path;
        }
    }
    false
}

/// Records a finding when the hook does not dedup discovered owning TS units.
fn check_dedups_owning_units(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let body = source_text(input.parsed());
    let dedup_present = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        line.contains("sort -u")
            || line.contains("sort --unique")
            || line.contains("uniq")
            || line.contains("awk '!seen")
    });
    if dedup_present {
        return;
    }
    results.push(G3CheckResult::new(
        DEDUPS_OWNING_UNITS_ID.to_owned(),
        G3Severity::Error,
        "hook does not dedup owning units".to_owned(),
        ".githooks/pre-commit must dedup discovered owning TS units (e.g. `sort -u`, `uniq`, or equivalent) before invoking `g3ts validate`."
            .to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook lacks a lenient skip for files without an owning unit.
fn check_skips_when_no_owning_unit(
    input: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let body = source_text(input.parsed());
    let lenient = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        trimmed.starts_with("continue")
            || trimmed.contains("[ -z ")
            || trimmed.contains("[ -n ")
            || trimmed.contains("[[ -z ")
            || trimmed.contains("[[ -n ")
    });
    if lenient {
        return;
    }
    results.push(G3CheckResult::new(
        SKIPS_WHEN_NO_OWNING_UNIT_ID.to_owned(),
        G3Severity::Error,
        "hook does not skip when no owning unit".to_owned(),
        ".githooks/pre-commit must silently skip TS-relevant staged files that have no owning adopted unit (lenient policy)."
            .to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Toolchain commands the hook must not invoke directly.
const FORBIDDEN_TOOLCHAIN_COMMANDS: &[&str] = &["tsc", "eslint", "prettier", "cspell", "stylelint"];

/// Pnpm command prefixes that are exempt from the forbidden-toolchain rule.
const PNPM_ALLOWED_PREFIXES: &[&str] = &[
    "pnpm install --frozen-lockfile",
    "pnpm exec drizzle-kit generate",
];

/// Records a finding when the hook directly invokes the TS toolchain.
fn check_no_toolchain_invocation(
    input: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    for line in &input.parsed().executable_lines {
        let name = line.command_name.as_str();
        if name == "pnpm" {
            let text = line.command_text.as_str();
            // Skip allowed contexts (lockfile-integrity inline check, drizzle generate).
            if PNPM_ALLOWED_PREFIXES
                .iter()
                .any(|prefix| text.starts_with(prefix) || text.contains(prefix))
            {
                continue;
            }
            results.push(G3CheckResult::new(
                NO_TOOLCHAIN_INVOCATION_ID.to_owned(),
                G3Severity::Error,
                "hook invokes pnpm directly".to_owned(),
                format!(
                    "`.githooks/pre-commit` invokes `{text}` directly. The hook must delegate all TypeScript toolchain work to `g3ts validate --path <unit> --staged`; only `pnpm install --frozen-lockfile` (lockfile integrity) and `pnpm exec drizzle-kit generate` (error-message hint) are allowed."
                ),
                Some(input.rel_path().to_owned()),
                Some(line.line_no),
            ));
            return;
        }
        if FORBIDDEN_TOOLCHAIN_COMMANDS.contains(&name) {
            results.push(G3CheckResult::new(
                NO_TOOLCHAIN_INVOCATION_ID.to_owned(),
                G3Severity::Error,
                format!("hook invokes {name} directly"),
                format!(
                    "`.githooks/pre-commit` invokes `{}` directly. The hook must delegate all TypeScript toolchain work to `g3ts validate --path <unit> --staged`.",
                    line.command_text
                ),
                Some(input.rel_path().to_owned()),
                Some(line.line_no),
            ));
            return;
        }
    }
}

/// Records a finding when the hook does not scan staged files for merge-conflict markers.
fn check_merge_conflict_scan(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let body = source_text(input.parsed());
    let has_marker_pattern = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        // Recognize either the literal seven-character form (`<<<<<<<`) or the
        // regex quantifier form (`<{7}`).
        trimmed.contains("<<<<<<<")
            || trimmed.contains("=======")
            || trimmed.contains(">>>>>>>")
            || trimmed.contains("<{7}")
            || trimmed.contains("={7}")
            || trimmed.contains(">{7}")
            || trimmed.contains("merge-conflict")
            || trimmed.contains("merge_conflict")
    });
    if has_marker_pattern {
        return;
    }
    results.push(G3CheckResult::new(
        MERGE_CONFLICT_SCAN_ID.to_owned(),
        G3Severity::Error,
        "pre-commit hook does not scan staged files for merge-conflict markers".to_owned(),
        "The pre-commit hook must scan staged files for merge-conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).".to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook does not run a gitleaks-equivalent secret scan.
fn check_gitleaks_scan(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let has_gitleaks = any_resolved_command(parsed, |command| {
        command.command_name() == "gitleaks"
            && command.args().iter().any(|arg| arg == "protect")
            && command.args().iter().any(|arg| arg == "--staged")
    });
    if has_gitleaks {
        return;
    }
    results.push(G3CheckResult::new(
        GITLEAKS_SCAN_ID.to_owned(),
        G3Severity::Error,
        "pre-commit hook does not run a secret scan over staged files".to_owned(),
        "The pre-commit hook must run `gitleaks protect --staged` (or an equivalent secret scan) over staged files.".to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook does not enforce a staged-file size cap.
fn check_file_size_cap(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let has_size_command = any_resolved_command(parsed, |command| {
        command.command_name() == "git"
            && command.args().first().is_some_and(|arg| arg == "cat-file")
            && command.args().iter().any(|arg| arg == "-s")
    });
    let body = source_text(parsed);
    let has_size_keyword = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        let lower = trimmed.to_ascii_lowercase();
        lower.contains("max_file_size")
            || lower.contains("file_size")
            || lower.contains("file size")
    });
    if has_size_command && has_size_keyword {
        return;
    }
    results.push(G3CheckResult::new(
        FILE_SIZE_CAP_ID.to_owned(),
        G3Severity::Error,
        "pre-commit hook does not enforce a staged-file size cap".to_owned(),
        "The pre-commit hook must enforce a staged-file size cap (e.g. via `git cat-file -s` and a configured maximum).".to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook does not enforce drizzle-migration consistency.
fn check_migration_consistency(
    input: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let body = source_text(input.parsed());
    let mentions_drizzle = has_uncommented_substring(&body, "drizzle/");
    let mentions_schema = has_uncommented_substring(&body, "db/schema/");
    if mentions_drizzle && mentions_schema {
        return;
    }
    results.push(G3CheckResult::new(
        MIGRATION_CONSISTENCY_ID.to_owned(),
        G3Severity::Error,
        "pre-commit hook does not enforce migration consistency".to_owned(),
        "The pre-commit hook must enforce migration consistency: forbid modifying existing `drizzle/*` files and require a new `drizzle/*.sql` file when `db/schema/*.ts` is staged.".to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Records a finding when the hook does not run a lockfile integrity check.
fn check_lockfile_integrity(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let has_pnpm_install = any_resolved_command(parsed, |command| {
        command.command_name() == "pnpm"
            && command.args().first().is_some_and(|arg| arg == "install")
            && command.args().iter().any(|arg| arg == "--frozen-lockfile")
    });
    let has_npm_dry_run = any_resolved_command(parsed, |command| {
        command.command_name() == "npm"
            && command.args().first().is_some_and(|arg| arg == "install")
            && command
                .args()
                .iter()
                .any(|arg| arg == "--package-lock-only")
            && command.args().iter().any(|arg| arg == "--dry-run")
    });
    let body = source_text(parsed);
    let mentions_package_json = has_uncommented_substring(&body, "package.json");
    if (has_pnpm_install || has_npm_dry_run) && mentions_package_json {
        return;
    }
    results.push(G3CheckResult::new(
        LOCKFILE_INTEGRITY_ID.to_owned(),
        G3Severity::Error,
        "pre-commit hook does not enforce lockfile integrity".to_owned(),
        "The pre-commit hook must run a lockfile integrity check for staged `package.json` (e.g. `pnpm install --frozen-lockfile` or `npm install --package-lock-only --dry-run`).".to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Joins all source lines of `parsed` into a single newline-delimited body.
fn source_text(parsed: &hook_shell_parser::types::ParsedShellScript) -> String {
    parsed
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns true when any non-comment line of `body` contains `needle`.
fn has_uncommented_substring(body: &str, needle: &str) -> bool {
    body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        line.contains(needle)
    })
}
