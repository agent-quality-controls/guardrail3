use std::collections::BTreeSet;

use g3ts_spelling_types::{
    G3TsSpellingDependencyDeclarationSnapshot, G3TsSpellingPackageScriptCommandSeparator,
    G3TsSpellingPackageScriptToolInvocation, G3TsSpellingPackageSurfaceSnapshot,
    G3TsSpellingPackageSurfaceState, G3TsSpellingSyncpackSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `parsed_package`: parsed package.
pub(crate) const fn parsed_package(
    package: &G3TsSpellingPackageSurfaceState,
) -> Option<&G3TsSpellingPackageSurfaceSnapshot> {
    match package {
        G3TsSpellingPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsSpellingPackageSurfaceState::Missing { .. }
        | G3TsSpellingPackageSurfaceState::Unreadable { .. }
        | G3TsSpellingPackageSurfaceState::ParseError { .. } => None,
    }
}

/// `package_rel_path`: package rel path.
pub(crate) fn package_rel_path(package: &G3TsSpellingPackageSurfaceState) -> &str {
    match package {
        G3TsSpellingPackageSurfaceState::Missing { rel_path }
        | G3TsSpellingPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsSpellingPackageSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsSpellingPackageSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// `parsed_syncpack`: parsed syncpack.
pub(crate) const fn parsed_syncpack(
    state: &G3TsSpellingSyncpackSurfaceState,
) -> Option<&g3ts_spelling_types::G3TsSpellingSyncpackSnapshot> {
    match state {
        G3TsSpellingSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsSpellingSyncpackSurfaceState::Missing { .. }
        | G3TsSpellingSyncpackSurfaceState::Unreadable { .. }
        | G3TsSpellingSyncpackSurfaceState::ParseError { .. } => None,
    }
}

/// `syncpack_rel_path`: syncpack rel path.
pub(crate) fn syncpack_rel_path(state: &G3TsSpellingSyncpackSurfaceState) -> &str {
    match state {
        G3TsSpellingSyncpackSurfaceState::Missing { rel_path }
        | G3TsSpellingSyncpackSurfaceState::Unreadable { rel_path, .. }
        | G3TsSpellingSyncpackSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsSpellingSyncpackSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// `package_has_dependency`: package has dependency.
pub(crate) fn package_has_dependency(
    package: &G3TsSpellingPackageSurfaceSnapshot,
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
    package: &'package G3TsSpellingPackageSurfaceSnapshot,
    dependency: &str,
) -> Vec<&'package G3TsSpellingDependencyDeclarationSnapshot> {
    package
        .dependency_declarations
        .iter()
        .filter(|declaration| declaration.name == dependency)
        .collect()
}

/// `script_invokes_cspell`: script invokes cspell.
pub(crate) fn script_invokes_cspell(
    package: &G3TsSpellingPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    package
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
        && script_has_no_or_separator(package, script_name)
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == script_name && cspell_invocation(invocation)
        })
}

/// `validate_runs_spellcheck`: validate runs spellcheck.
pub(crate) fn validate_runs_spellcheck(package: &G3TsSpellingPackageSurfaceSnapshot) -> bool {
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
            && (invocation.preceded_by == Some(G3TsSpellingPackageScriptCommandSeparator::Or)
                || invocation.followed_by == Some(G3TsSpellingPackageScriptCommandSeparator::Or))
    }) {
        return false;
    }
    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name) && cspell_invocation(invocation)
    })
}

/// `info`: info.
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

/// `error`: error.
pub(crate) fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}

/// `cspell_invocation`: cspell invocation.
fn cspell_invocation(invocation: &G3TsSpellingPackageScriptToolInvocation) -> bool {
    cspell_args(invocation)
        .is_some_and(|args| !cspell_args_disable_exit_code(args) && cspell_args_have_target(args))
}

/// `script_has_no_or_separator`: script has no or separator.
fn script_has_no_or_separator(
    package: &G3TsSpellingPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    package
        .script_tool_invocations
        .iter()
        .filter(|invocation| invocation.script_name == script_name)
        .all(|invocation| {
            invocation.preceded_by != Some(G3TsSpellingPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsSpellingPackageScriptCommandSeparator::Or)
        })
}

/// `cspell_args`: cspell args.
fn cspell_args(invocation: &G3TsSpellingPackageScriptToolInvocation) -> Option<&[String]> {
    if invocation.executable == "cspell" && original_command_starts_with(invocation, "cspell") {
        return Some(&invocation.args);
    }
    None
}

/// Returns true when cspell args include a positional file, directory, or glob target.
fn cspell_args_have_target(args: &[String]) -> bool {
    let mut idx = 0;
    if args.first().is_some_and(|arg| arg == "lint") {
        idx = 1;
    } else if args
        .first()
        .is_some_and(|arg| cspell_non_lint_subcommand(arg))
    {
        return false;
    }
    while idx < args.len() {
        let Some(arg) = args.get(idx) else {
            break;
        };
        if arg == "--" {
            return args
                .get(idx.saturating_add(1)..)
                .is_some_and(|rest| rest.iter().any(|candidate| !candidate.trim().is_empty()));
        }
        if arg.starts_with("--") {
            if !arg.contains('=') && cspell_option_takes_value(arg) {
                idx = idx.saturating_add(2);
            } else {
                idx = idx.saturating_add(1);
            }
            continue;
        }
        if arg.starts_with('-') {
            if cspell_option_takes_value(arg) {
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

/// Returns true when the `CSpell` args disable failure exit status.
fn cspell_args_disable_exit_code(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--no-exit-code")
}

/// Returns true for `CSpell` subcommands that are not project spellcheck commands.
fn cspell_non_lint_subcommand(arg: &str) -> bool {
    matches!(arg, "check" | "trace")
}

/// `CSpell` flags whose value is the next argv token rather than a checked target.
fn cspell_option_takes_value(arg: &str) -> bool {
    matches!(
        arg,
        "-c" | "--config"
            | "--locale"
            | "--language-id"
            | "--dictionary"
            | "--dictionaries"
            | "--exclude"
            | "--file-list"
            | "--cache-location"
            | "--cache-strategy"
            | "--reporter"
            | "--issue-template"
            | "--root"
            | "--stop-config-search-at"
    )
}

/// `reachable_script_names`: reachable script names.
fn reachable_script_names(
    package: &G3TsSpellingPackageSurfaceSnapshot,
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

/// `package_script_target`: package script target.
fn package_script_target(invocation: &G3TsSpellingPackageScriptToolInvocation) -> Option<String> {
    if invocation.executable == "package-script" {
        let target = invocation.args.first()?;
        if target == "spellcheck"
            && invocation_uses_package_manager_script_invocation(invocation, target)
        {
            return Some(target.clone());
        }
        return None;
    }
    if invocation.executable == "spellcheck"
        && invocation_uses_package_manager_script_invocation(invocation, "spellcheck")
    {
        return Some("spellcheck".to_owned());
    }
    None
}

/// Returns true when the original command is an approved package-manager script invocation.
fn invocation_uses_package_manager_script_invocation(
    invocation: &G3TsSpellingPackageScriptToolInvocation,
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
    invocation: &G3TsSpellingPackageScriptToolInvocation,
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
