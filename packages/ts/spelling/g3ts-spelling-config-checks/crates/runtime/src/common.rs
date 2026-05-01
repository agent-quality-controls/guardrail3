use std::collections::BTreeSet;

use g3ts_spelling_types::{
    G3TsSpellingPackageScriptCommandSeparator, G3TsSpellingPackageScriptToolInvocation,
    G3TsSpellingPackageSurfaceSnapshot, G3TsSpellingPackageSurfaceState,
    G3TsSpellingSyncpackSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn parsed_package(
    package: &G3TsSpellingPackageSurfaceState,
) -> Option<&G3TsSpellingPackageSurfaceSnapshot> {
    match package {
        G3TsSpellingPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsSpellingPackageSurfaceState::Missing { .. }
        | G3TsSpellingPackageSurfaceState::Unreadable { .. }
        | G3TsSpellingPackageSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn package_rel_path(package: &G3TsSpellingPackageSurfaceState) -> Option<&str> {
    match package {
        G3TsSpellingPackageSurfaceState::Missing { rel_path }
        | G3TsSpellingPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsSpellingPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsSpellingPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn parsed_syncpack(
    state: &G3TsSpellingSyncpackSurfaceState,
) -> Option<&g3ts_spelling_types::G3TsSpellingSyncpackSnapshot> {
    match state {
        G3TsSpellingSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsSpellingSyncpackSurfaceState::Missing { .. }
        | G3TsSpellingSyncpackSurfaceState::Unreadable { .. }
        | G3TsSpellingSyncpackSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn syncpack_rel_path(state: &G3TsSpellingSyncpackSurfaceState) -> Option<&str> {
    match state {
        G3TsSpellingSyncpackSurfaceState::Missing { rel_path }
        | G3TsSpellingSyncpackSurfaceState::Unreadable { rel_path, .. }
        | G3TsSpellingSyncpackSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsSpellingSyncpackSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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

fn cspell_invocation(invocation: &G3TsSpellingPackageScriptToolInvocation) -> bool {
    cspell_args(invocation).is_some()
}

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

fn cspell_args(invocation: &G3TsSpellingPackageScriptToolInvocation) -> Option<&[String]> {
    if invocation.executable == "cspell" {
        return Some(&invocation.args);
    }
    if matches!(
        invocation.executable.as_str(),
        "pnpm" | "npm" | "yarn" | "bun" | "npx" | "bunx"
    ) {
        let (tool, args) = invocation.args.split_first()?;
        if tool == "cspell" {
            return Some(args);
        }
    }
    None
}

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

fn package_script_target(invocation: &G3TsSpellingPackageScriptToolInvocation) -> Option<String> {
    if invocation.executable == "package-script" {
        return invocation.args.first().cloned();
    }
    if invocation.executable == "spellcheck" {
        return Some("spellcheck".to_owned());
    }
    if matches!(invocation.executable.as_str(), "pnpm" | "yarn" | "bun") {
        return invocation
            .args
            .first()
            .filter(|script_name| *script_name == "spellcheck")
            .cloned();
    }
    None
}
