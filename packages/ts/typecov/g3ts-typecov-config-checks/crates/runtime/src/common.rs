use std::collections::BTreeSet;

use g3ts_typecov_types::{
    G3TsTypecovPackageScriptCommandSeparator, G3TsTypecovPackageScriptToolInvocation,
    G3TsTypecovPackageSurfaceSnapshot, G3TsTypecovPackageSurfaceState,
    G3TsTypecovSyncpackSurfaceState,
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

/// Returns true when `script_name` invokes `type-coverage --at-least 100` fail-closed.
pub(crate) fn script_invokes_type_coverage(
    package: &G3TsTypecovPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    package
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
        && script_has_no_or_separator(package, script_name)
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == script_name && type_coverage_invocation_at_100(invocation)
        })
}

/// Returns true when the `validate` script reaches a fail-closed `typecov` invocation.
pub(crate) fn validate_runs_typecov(package: &G3TsTypecovPackageSurfaceSnapshot) -> bool {
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
        reachable.contains(&invocation.script_name) && type_coverage_invocation_at_100(invocation)
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

/// Returns true when the invocation is `type-coverage --at-least 100`.
fn type_coverage_invocation_at_100(invocation: &G3TsTypecovPackageScriptToolInvocation) -> bool {
    let Some(args) = type_coverage_args(invocation) else {
        return false;
    };
    args.windows(2).any(|window| {
        window.first().is_some_and(|arg| arg == "--at-least")
            && window.get(1).is_some_and(|arg| arg == "100")
    })
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
    if invocation.executable == "type-coverage" {
        return Some(&invocation.args);
    }
    if matches!(
        invocation.executable.as_str(),
        "pnpm" | "npm" | "yarn" | "bun" | "npx" | "bunx"
    ) {
        let (tool, args) = invocation.args.split_first()?;
        if tool == "type-coverage" {
            return Some(args);
        }
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
        return invocation.args.first().cloned();
    }
    if invocation.executable == "typecov" {
        return Some("typecov".to_owned());
    }
    if matches!(invocation.executable.as_str(), "pnpm" | "yarn" | "bun") {
        return invocation
            .args
            .first()
            .filter(|script_name| *script_name == "typecov")
            .cloned();
    }
    None
}
