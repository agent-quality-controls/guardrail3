use std::collections::BTreeSet;

use g3ts_fmt_types::{
    G3TsFmtPackageScriptCommandSeparator, G3TsFmtPackageScriptToolInvocation,
    G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState, G3TsFmtSyncpackSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn parsed_package(
    package: &G3TsFmtPackageSurfaceState,
) -> Option<&G3TsFmtPackageSurfaceSnapshot> {
    match package {
        G3TsFmtPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsFmtPackageSurfaceState::Missing { .. }
        | G3TsFmtPackageSurfaceState::Unreadable { .. }
        | G3TsFmtPackageSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn package_rel_path(package: &G3TsFmtPackageSurfaceState) -> Option<&str> {
    match package {
        G3TsFmtPackageSurfaceState::Missing { rel_path }
        | G3TsFmtPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsFmtPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsFmtPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn parsed_syncpack(
    state: &G3TsFmtSyncpackSurfaceState,
) -> Option<&g3ts_fmt_types::G3TsFmtSyncpackSnapshot> {
    match state {
        G3TsFmtSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsFmtSyncpackSurfaceState::Missing { .. }
        | G3TsFmtSyncpackSurfaceState::Unreadable { .. }
        | G3TsFmtSyncpackSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn syncpack_rel_path(state: &G3TsFmtSyncpackSurfaceState) -> Option<&str> {
    match state {
        G3TsFmtSyncpackSurfaceState::Missing { rel_path }
        | G3TsFmtSyncpackSurfaceState::Unreadable { rel_path, .. }
        | G3TsFmtSyncpackSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsFmtSyncpackSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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

fn prettier_invocation_has_arg(
    invocation: &G3TsFmtPackageScriptToolInvocation,
    required_arg: &str,
) -> bool {
    let Some(args) = prettier_args(invocation) else {
        return false;
    };
    args.iter().any(|arg| arg == required_arg)
}

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
