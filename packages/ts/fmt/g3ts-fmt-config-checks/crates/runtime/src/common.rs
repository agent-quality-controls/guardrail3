use std::collections::BTreeSet;

use g3ts_fmt_types::{
    G3TsFmtDependencyDeclarationSnapshot, G3TsFmtPackageScriptCommandSeparator,
    G3TsFmtPackageScriptToolInvocation, G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState,
    G3TsFmtSyncpackSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Returns the parsed package snapshot when the surface state is `Parsed`.
pub(crate) const fn parsed_package(
    package: &G3TsFmtPackageSurfaceState,
) -> Option<&G3TsFmtPackageSurfaceSnapshot> {
    match package {
        G3TsFmtPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsFmtPackageSurfaceState::Missing { .. }
        | G3TsFmtPackageSurfaceState::Unreadable { .. }
        | G3TsFmtPackageSurfaceState::ParseError { .. } => None,
    }
}

/// Returns the workspace-relative path of the package surface for any state.
pub(crate) fn package_rel_path(package: &G3TsFmtPackageSurfaceState) -> &str {
    match package {
        G3TsFmtPackageSurfaceState::Missing { rel_path }
        | G3TsFmtPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsFmtPackageSurfaceState::ParseError { rel_path, .. } => rel_path.as_str(),
        G3TsFmtPackageSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Returns the parsed syncpack snapshot when the surface state is `Parsed`.
pub(crate) const fn parsed_syncpack(
    state: &G3TsFmtSyncpackSurfaceState,
) -> Option<&g3ts_fmt_types::G3TsFmtSyncpackSnapshot> {
    match state {
        G3TsFmtSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsFmtSyncpackSurfaceState::Missing { .. }
        | G3TsFmtSyncpackSurfaceState::Unreadable { .. }
        | G3TsFmtSyncpackSurfaceState::ParseError { .. } => None,
    }
}

/// Returns the workspace-relative path of the syncpack surface for any state.
pub(crate) fn syncpack_rel_path(state: &G3TsFmtSyncpackSurfaceState) -> &str {
    match state {
        G3TsFmtSyncpackSurfaceState::Parsed { snapshot } => snapshot.rel_path.as_str(),
        G3TsFmtSyncpackSurfaceState::Missing { rel_path }
        | G3TsFmtSyncpackSurfaceState::Unreadable { rel_path, .. }
        | G3TsFmtSyncpackSurfaceState::ParseError { rel_path, .. } => rel_path,
    }
}

/// Returns true when the package directly depends on `dependency`.
pub(crate) fn package_has_dependency(
    package: &G3TsFmtPackageSurfaceSnapshot,
    dependency: &str,
) -> bool {
    package
        .dependencies
        .iter()
        .chain(package.dev_dependencies.iter())
        .any(|candidate| candidate == dependency)
}

/// Returns the dependency declarations for `dependency`.
pub(crate) fn package_dependency_declarations<'package>(
    package: &'package G3TsFmtPackageSurfaceSnapshot,
    dependency: &str,
) -> Vec<&'package G3TsFmtDependencyDeclarationSnapshot> {
    package
        .dependency_declarations
        .iter()
        .filter(|declaration| declaration.name == dependency)
        .collect()
}

/// Returns true when `script_name` invokes prettier with `required_arg` fail-closed.
pub(crate) fn script_invokes_prettier(
    package: &G3TsFmtPackageSurfaceSnapshot,
    script_name: &str,
    required_arg: &str,
) -> bool {
    package
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
        && script_has_no_or_separator(package, script_name)
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == script_name
                && prettier_invocation_has_arg_and_target(invocation, required_arg)
        })
}

/// Returns true when `validate` reaches a fail-closed `prettier --check` invocation.
pub(crate) fn validate_runs_format_check(package: &G3TsFmtPackageSurfaceSnapshot) -> bool {
    if !package.script_names.iter().any(|name| name == "validate") {
        return false;
    }
    let reachable = reachable_script_names(package, "validate");
    if package
        .script_parse_blockers
        .iter()
        .any(|blocker| reachable.contains(&blocker.script_name))
    {
        return false;
    }
    if package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && (invocation.preceded_by == Some(G3TsFmtPackageScriptCommandSeparator::Or)
                || invocation.followed_by == Some(G3TsFmtPackageScriptCommandSeparator::Or))
    }) {
        return false;
    }
    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && prettier_invocation_has_arg_and_target(invocation, "--check")
    })
}

/// Builds an inventoried info-severity check result.
pub(crate) fn info(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
    .into_inventory()
}

/// Builds an error-severity check result.
pub(crate) fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    let severity = G3Severity::Error;
    G3CheckResult::new(
        id.to_owned(),
        severity,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}

/// Returns true when the prettier invocation contains `required_arg` and a checked target.
fn prettier_invocation_has_arg_and_target(
    invocation: &G3TsFmtPackageScriptToolInvocation,
    required_arg: &str,
) -> bool {
    let Some(args) = prettier_args(invocation) else {
        return false;
    };
    args.iter().any(|arg| arg == required_arg) && prettier_args_have_target(args, required_arg)
}

/// Returns true when no invocation in `script_name` uses an `||` separator.
fn script_has_no_or_separator(package: &G3TsFmtPackageSurfaceSnapshot, script_name: &str) -> bool {
    package
        .script_tool_invocations
        .iter()
        .filter(|invocation| invocation.script_name == script_name)
        .all(|invocation| {
            invocation.preceded_by != Some(G3TsFmtPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsFmtPackageScriptCommandSeparator::Or)
        })
}

/// Returns true when prettier args include a positional file, directory, or glob target.
fn prettier_args_have_target(args: &[String], required_arg: &str) -> bool {
    let mut idx = 0;
    while idx < args.len() {
        let Some(arg) = args.get(idx) else {
            break;
        };
        if arg == "--" {
            return args
                .get(idx.saturating_add(1)..)
                .is_some_and(|rest| rest.iter().any(|candidate| !candidate.trim().is_empty()));
        }
        if arg == required_arg {
            idx = idx.saturating_add(1);
            continue;
        }
        if let Some(option) = arg.strip_prefix("--") {
            if option.contains('=') || prettier_boolean_option(arg) {
                idx = idx.saturating_add(1);
            } else {
                idx = idx.saturating_add(2);
            }
            continue;
        }
        if arg.starts_with('-') {
            if prettier_option_takes_value(arg) {
                idx = idx.saturating_add(2);
            } else {
                idx = idx.saturating_add(1);
            }
            continue;
        }
        return !arg.trim().is_empty();
    }
    false
}

/// Returns true for Prettier options whose value is the next argv token.
fn prettier_option_takes_value(arg: &str) -> bool {
    PRETTIER_VALUE_OPTIONS.contains(&arg)
}

/// Returns true for Prettier boolean options that do not consume the next argv token.
fn prettier_boolean_option(arg: &str) -> bool {
    PRETTIER_BOOLEAN_OPTIONS.contains(&arg)
}

/// Prettier options whose value is supplied by the next argv token.
const PRETTIER_VALUE_OPTIONS: &[&str] = &[
    "--arrow-parens",
    "--cache-location",
    "--cache-strategy",
    "--config",
    "--config-precedence",
    "--embedded-language-formatting",
    "--end-of-line",
    "--find-config-path",
    "--html-whitespace-sensitivity",
    "--ignore-path",
    "--log-level",
    "--object-wrap",
    "--parser",
    "--plugin",
    "--plugin-search-dir",
    "--print-width",
    "--prose-wrap",
    "--quote-props",
    "--range-end",
    "--range-start",
    "--stdin-filepath",
    "--tab-width",
    "--trailing-comma",
];

/// Prettier boolean options that do not consume the next argv token.
const PRETTIER_BOOLEAN_OPTIONS: &[&str] = &[
    "--bracket-same-line",
    "--bracket-spacing",
    "--cache",
    "--check",
    "--debug-check",
    "--ignore-unknown",
    "--jsx-single-quote",
    "--list-different",
    "--no-bracket-spacing",
    "--no-config",
    "--no-editorconfig",
    "--no-error-on-unmatched-pattern",
    "--no-semi",
    "--no-vue-indent-script-and-style",
    "--require-pragma",
    "--semi",
    "--single-attribute-per-line",
    "--single-quote",
    "--support-info",
    "--version",
    "--vue-indent-script-and-style",
    "--with-node-modules",
    "--write",
];

/// Returns the prettier args slice when the invocation is `prettier` directly or via a runner.
fn prettier_args(invocation: &G3TsFmtPackageScriptToolInvocation) -> Option<&[String]> {
    if invocation.executable == "prettier" && original_command_starts_with(invocation, "prettier") {
        return Some(&invocation.args);
    }
    None
}

/// Returns script names transitively reachable from `root_script_name`.
fn reachable_script_names(
    package: &G3TsFmtPackageSurfaceSnapshot,
    root_script_name: &str,
) -> BTreeSet<String> {
    let mut reachable = BTreeSet::from([root_script_name.to_owned()]);
    let mut pending = vec![root_script_name.to_owned()];
    while let Some(script_name) = pending.pop() {
        for invocation in package
            .script_tool_invocations
            .iter()
            .filter(|invocation| invocation.script_name == script_name)
        {
            let Some(target) = package_script_target(invocation) else {
                continue;
            };
            if reachable.insert(target.clone()) {
                pending.push(target);
            }
        }
    }
    reachable
}

/// Returns the script name targeted by a package-script invocation, when applicable.
fn package_script_target(invocation: &G3TsFmtPackageScriptToolInvocation) -> Option<String> {
    if invocation.executable == "package-script" {
        let target = invocation.args.first()?;
        if target == "format:check"
            && invocation_uses_package_manager_script_invocation(invocation, target)
        {
            return Some(target.clone());
        }
        return None;
    }
    if invocation.executable == "format:check"
        && invocation_uses_package_manager_script_invocation(invocation, "format:check")
    {
        return Some("format:check".to_owned());
    }
    None
}

/// Returns true when the original command is an approved package-manager script invocation.
fn invocation_uses_package_manager_script_invocation(
    invocation: &G3TsFmtPackageScriptToolInvocation,
    script_name: &str,
) -> bool {
    let tokens = command_tokens(&invocation.invocation);
    if tokens.len() == 2
        && matches!(tokens.first().copied(), Some("pnpm" | "yarn" | "bun"))
        && tokens.get(1).is_some_and(|token| *token == script_name)
    {
        return true;
    }
    tokens.len() == 3
        && matches!(tokens.first().copied(), Some("pnpm" | "yarn" | "bun"))
        && tokens.get(1).is_some_and(|token| *token == "run")
        && tokens.get(2).is_some_and(|token| *token == script_name)
}

/// Returns true when the original command starts with `command` after env wrappers.
fn original_command_starts_with(
    invocation: &G3TsFmtPackageScriptToolInvocation,
    command: &str,
) -> bool {
    command_tokens(&invocation.invocation)
        .first()
        .is_some_and(|token| *token == command)
}

/// Returns command tokens after env wrappers and env assignments.
fn command_tokens(invocation: &str) -> Vec<&str> {
    invocation
        .split_whitespace()
        .skip_while(|token| matches!(*token, "env" | "cross-env") || token.contains('='))
        .collect()
}
