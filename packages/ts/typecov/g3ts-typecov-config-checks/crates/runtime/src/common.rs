use std::collections::BTreeSet;

use g3ts_typecov_types::{
    G3TsTypecovContractInput, G3TsTypecovDependencyDeclarationSnapshot,
    G3TsTypecovPackageScriptCommandSeparator, G3TsTypecovPackageScriptToolInvocation,
    G3TsTypecovPackageSurfaceSnapshot, G3TsTypecovPackageSurfaceState,
    G3TsTypecovPolicySurfaceState, G3TsTypecovSyncpackSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Returns the parsed package snapshot when the surface state is `Parsed`.
pub(crate) const fn parsed_package(
    package: &G3TsTypecovPackageSurfaceState,
) -> Option<&G3TsTypecovPackageSurfaceSnapshot> {
    if let G3TsTypecovPackageSurfaceState::Parsed { snapshot } = package {
        Some(snapshot)
    } else {
        None
    }
}

/// Returns the workspace-relative path of the package surface for any state.
pub(crate) fn package_rel_path(package: &G3TsTypecovPackageSurfaceState) -> &str {
    match package {
        G3TsTypecovPackageSurfaceState::Missing { rel_path }
        | G3TsTypecovPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsTypecovPackageSurfaceState::ParseError { rel_path, .. } => rel_path.as_str(),
        G3TsTypecovPackageSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Returns the parsed syncpack snapshot when the surface state is `Parsed`.
pub(crate) const fn parsed_syncpack(
    state: &G3TsTypecovSyncpackSurfaceState,
) -> Option<&g3ts_typecov_types::G3TsTypecovSyncpackSnapshot> {
    match state {
        G3TsTypecovSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsTypecovSyncpackSurfaceState::Missing { .. }
        | G3TsTypecovSyncpackSurfaceState::Unreadable { .. }
        | G3TsTypecovSyncpackSurfaceState::ParseError { .. } => None,
    }
}

/// Returns the workspace-relative path of the syncpack surface for any state.
pub(crate) fn syncpack_rel_path(state: &G3TsTypecovSyncpackSurfaceState) -> &str {
    match state {
        G3TsTypecovSyncpackSurfaceState::Parsed { snapshot } => snapshot.rel_path.as_str(),
        G3TsTypecovSyncpackSurfaceState::Missing { rel_path }
        | G3TsTypecovSyncpackSurfaceState::Unreadable { rel_path, .. }
        | G3TsTypecovSyncpackSurfaceState::ParseError { rel_path, .. } => rel_path,
    }
}

/// Returns the parsed typecov minimum for policy-backed checks.
pub(crate) const fn policy_minimum(contract: &G3TsTypecovContractInput) -> Option<u8> {
    match &contract.typecov_policy {
        G3TsTypecovPolicySurfaceState::Parsed { snapshot } => Some(snapshot.minimum),
        G3TsTypecovPolicySurfaceState::Missing { .. }
        | G3TsTypecovPolicySurfaceState::Unreadable { .. }
        | G3TsTypecovPolicySurfaceState::ParseError { .. }
        | G3TsTypecovPolicySurfaceState::MissingTypecovPolicy { .. } => None,
    }
}

/// Returns the workspace-relative path of the typecov policy surface for any state.
pub(crate) fn policy_rel_path(state: &G3TsTypecovPolicySurfaceState) -> &str {
    match state {
        G3TsTypecovPolicySurfaceState::Parsed { snapshot } => snapshot.rel_path.as_str(),
        G3TsTypecovPolicySurfaceState::Missing { rel_path }
        | G3TsTypecovPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsTypecovPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsTypecovPolicySurfaceState::MissingTypecovPolicy { rel_path } => rel_path,
    }
}

/// Returns true when the package directly depends on `dependency` in any dependency list.
pub(crate) fn package_has_dependency(
    package: &G3TsTypecovPackageSurfaceSnapshot,
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
    package: &'package G3TsTypecovPackageSurfaceSnapshot,
    dependency: &str,
) -> Vec<&'package G3TsTypecovDependencyDeclarationSnapshot> {
    package
        .dependency_declarations
        .iter()
        .filter(|declaration| declaration.name == dependency)
        .collect()
}

/// Returns true when `script_name` invokes `type-coverage --strict --at-least <n>` fail-closed.
pub(crate) fn script_invokes_type_coverage(
    package: &G3TsTypecovPackageSurfaceSnapshot,
    script_name: &str,
    minimum: u8,
) -> bool {
    package
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
        && script_has_no_or_separator(package, script_name)
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == script_name
                && type_coverage_invocation_satisfies_policy(invocation, minimum)
        })
}

/// Returns true when the `validate` script reaches a fail-closed `typecov` invocation.
pub(crate) fn validate_runs_typecov(
    package: &G3TsTypecovPackageSurfaceSnapshot,
    minimum: u8,
) -> bool {
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
            && (invocation.preceded_by == Some(G3TsTypecovPackageScriptCommandSeparator::Or)
                || invocation.followed_by == Some(G3TsTypecovPackageScriptCommandSeparator::Or))
    }) {
        return false;
    }
    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && type_coverage_invocation_satisfies_policy(invocation, minimum)
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

/// Returns true when the invocation is `type-coverage --strict --at-least <n>`.
fn type_coverage_invocation_satisfies_policy(
    invocation: &G3TsTypecovPackageScriptToolInvocation,
    minimum: u8,
) -> bool {
    let Some(args) = type_coverage_args(invocation) else {
        return false;
    };
    let thresholds = type_coverage_thresholds(args);
    let Some(thresholds) = thresholds else {
        return false;
    };
    args.iter().any(|arg| arg == "--strict")
        && !thresholds.is_empty()
        && thresholds
            .iter()
            .all(|threshold| *threshold >= minimum && *threshold <= 100)
}

/// Returns every `--at-least` threshold, rejecting missing, invalid, or malformed values.
fn type_coverage_thresholds(args: &[String]) -> Option<Vec<u8>> {
    let mut thresholds = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        let arg = args.get(idx)?;
        if arg == "--at-least" {
            let value = args.get(idx.saturating_add(1))?;
            thresholds.push(value.parse::<u8>().ok()?);
            idx = idx.saturating_add(2);
            continue;
        }
        if let Some(value) = arg.strip_prefix("--at-least=") {
            thresholds.push(value.parse::<u8>().ok()?);
        }
        idx = idx.saturating_add(1);
    }
    Some(thresholds)
}

/// Returns true when no invocation in `script_name` uses an `||` separator.
fn script_has_no_or_separator(
    package: &G3TsTypecovPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    package
        .script_tool_invocations
        .iter()
        .filter(|invocation| invocation.script_name == script_name)
        .all(|invocation| {
            invocation.preceded_by != Some(G3TsTypecovPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsTypecovPackageScriptCommandSeparator::Or)
        })
}

/// Returns the args slice when the invocation invokes `type-coverage` directly or through a runner.
fn type_coverage_args(invocation: &G3TsTypecovPackageScriptToolInvocation) -> Option<&[String]> {
    if invocation.executable == "type-coverage"
        && original_command_starts_with(invocation, "type-coverage")
    {
        return Some(&invocation.args);
    }
    None
}

/// Returns the script names transitively reachable from `root_script_name`.
fn reachable_script_names(
    package: &G3TsTypecovPackageSurfaceSnapshot,
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
fn package_script_target(invocation: &G3TsTypecovPackageScriptToolInvocation) -> Option<String> {
    if invocation.executable == "package-script" {
        let target = invocation.args.first()?;
        if target == "typecov"
            && invocation_uses_package_manager_script_invocation(invocation, target)
        {
            return Some(target.clone());
        }
        return None;
    }
    if invocation.executable == "typecov"
        && invocation_uses_package_manager_script_invocation(invocation, "typecov")
    {
        return Some("typecov".to_owned());
    }
    None
}

/// Returns true when the original command is an approved package-manager script invocation.
fn invocation_uses_package_manager_script_invocation(
    invocation: &G3TsTypecovPackageScriptToolInvocation,
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
    invocation: &G3TsTypecovPackageScriptToolInvocation,
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
