use g3ts_astro_setup_types::{
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroSetupIntegrationContractInput,
};
use guardrail3_check_types::G3CheckResult;
use std::collections::BTreeSet;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/validate-script";
/// Static rule data.
const VALIDATION_LIKE_SCRIPTS: [&str; 7] = [
    "prevalidate",
    "postvalidate",
    "check",
    "verify",
    "ci",
    "precommit",
    "lint:all",
];

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    let Some(package) = crate::support::parsed_package(&contract.package) else {
        results.push(crate::support::error(
            ID,
            "Astro app validate script cannot be checked",
            "The Astro validate script contract could not be checked because `package.json` was not parsed. Restore the app package manifest and add a fail-closed `validate` script.".to_owned(),
            Some(rel_path),
        ));
        return;
    };

    if validate_contract_is_satisfied(package) {
        results.push(crate::support::info(
            ID,
            "Astro app validate script runs every required validator",
            format!(
                "`{}` defines a fail-closed `validate` script that reaches ESLint, Syncpack, `astro check`, and `astro build`. This keeps delegated Astro checks executable through one standard app command.",
                package.rel_path
            ),
            &package.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro app validate script is missing required validators",
        format!(
            "`{}` must define one fail-closed `validate` script. It must invoke ESLint, `syncpack lint`, `astro check`, and `astro build`, either directly or through parseable package-manager `run` child scripts. Remove or fix unsafe validation lifecycle or sibling scripts (`prevalidate`, `postvalidate`, `check`, `verify`, `ci`, `precommit`, `lint:all`). Do not hide failures with `|| true` or shell syntax the package-script parser reports as unsupported.",
            package.rel_path
        ),
        Some(&package.rel_path),
    ));
}

/// Internal helper used by the rule.
fn validate_contract_is_satisfied(package: &G3TsAstroPackageSurfaceSnapshot) -> bool {
    if !package.script_names.iter().any(|name| name == "validate") {
        return false;
    }
    if !script_graph_is_safe(package, "validate") {
        return false;
    }
    if VALIDATION_LIKE_SCRIPTS.iter().any(|script_name| {
        script_exists(package, script_name) && !script_graph_is_safe(package, script_name)
    }) {
        return false;
    }

    let reachable = reachable_script_names(package, "validate");
    reachable_safely_runs_executable(package, &reachable, "eslint")
        && reachable_safely_runs_tool(package, &reachable, "syncpack", "lint")
        && reachable_safely_runs_tool(package, &reachable, "astro", "check")
        && reachable_safely_runs_tool(package, &reachable, "astro", "build")
}

/// Internal helper used by the rule.
fn script_exists(package: &G3TsAstroPackageSurfaceSnapshot, script_name: &str) -> bool {
    package.script_names.iter().any(|name| name == script_name)
}

/// Internal helper used by the rule.
fn script_graph_is_safe(package: &G3TsAstroPackageSurfaceSnapshot, script_name: &str) -> bool {
    let reachable = reachable_script_names(package, script_name);
    if reachable.is_empty() || !reachable.iter().any(|name| name == script_name) {
        return false;
    }
    if package
        .script_parse_blockers
        .iter()
        .any(|blocker| reachable.contains(&blocker.script_name))
    {
        return false;
    }
    if package.script_commands.iter().any(|command| {
        reachable.contains(&command.script_name)
            && command.preceded_by == Some(G3TsAstroPackageScriptCommandSeparator::Or)
    }) {
        return false;
    }
    if package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && (invocation.preceded_by == Some(G3TsAstroPackageScriptCommandSeparator::Or)
                || invocation.followed_by == Some(G3TsAstroPackageScriptCommandSeparator::Or))
    }) {
        return false;
    }

    package
        .script_tool_invocations
        .iter()
        .filter(|invocation| reachable.contains(&invocation.script_name))
        .filter_map(package_script_target)
        .all(|target| script_exists(package, &target))
}

/// Internal helper used by the rule.
fn reachable_script_names(
    package: &G3TsAstroPackageSurfaceSnapshot,
    root_script_name: &str,
) -> BTreeSet<String> {
    if !script_exists(package, root_script_name) {
        return BTreeSet::new();
    }

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

/// Internal helper used by the rule.
fn package_script_target(invocation: &G3TsAstroPackageScriptToolInvocation) -> Option<String> {
    if invocation.executable != "package-script" {
        return None;
    }

    invocation
        .args
        .first()
        .map(|script_name| script_name.trim())
        .filter(|script_name| !script_name.is_empty())
        .map(str::to_owned)
}

/// Internal helper used by the rule.
fn reachable_safely_runs_executable(
    package: &G3TsAstroPackageSurfaceSnapshot,
    reachable: &BTreeSet<String>,
    executable: &str,
) -> bool {
    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && invocation.executable == executable
            && invocation.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
            && invocation.followed_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}

/// Internal helper used by the rule.
fn reachable_safely_runs_tool(
    package: &G3TsAstroPackageSurfaceSnapshot,
    reachable: &BTreeSet<String>,
    executable: &str,
    first_arg: &str,
) -> bool {
    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && invocation.executable == executable
            && invocation.args.first().is_some_and(|arg| arg == first_arg)
            && invocation.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
            && invocation.followed_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}
