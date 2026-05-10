use std::collections::BTreeSet;

use g3ts_fmt_types::{
    G3TsFmtPackageScriptCommandSeparator, G3TsFmtPackageScriptToolInvocation,
    G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState, G3TsFmtSyncpackSurfaceState,
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
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == script_name
                && prettier_invocation_has_arg(invocation, required_arg)
                && invocation.preceded_by != Some(G3TsFmtPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsFmtPackageScriptCommandSeparator::Or)
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
            && prettier_invocation_has_arg(invocation, "--check")
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

/// Returns true when the prettier invocation contains `required_arg`.
fn prettier_invocation_has_arg(
    invocation: &G3TsFmtPackageScriptToolInvocation,
    required_arg: &str,
) -> bool {
    let Some(args) = prettier_args(invocation) else {
        return false;
    };
    args.iter().any(|arg| arg == required_arg)
}

/// Returns the prettier args slice when the invocation is `prettier` directly or via a runner.
fn prettier_args(invocation: &G3TsFmtPackageScriptToolInvocation) -> Option<&[String]> {
    if invocation.executable == "prettier" {
        return Some(&invocation.args);
    }
    if matches!(
        invocation.executable.as_str(),
        "pnpm" | "npm" | "yarn" | "bun" | "npx" | "bunx"
    ) {
        let (tool, args) = invocation.args.split_first()?;
        if tool == "prettier" {
            return Some(args);
        }
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
        return invocation.args.first().cloned();
    }
    if invocation.executable == "format:check" {
        return Some("format:check".to_owned());
    }
    if matches!(invocation.executable.as_str(), "pnpm" | "yarn" | "bun") {
        return invocation
            .args
            .first()
            .filter(|script_name| *script_name == "format:check")
            .cloned();
    }
    None
}
