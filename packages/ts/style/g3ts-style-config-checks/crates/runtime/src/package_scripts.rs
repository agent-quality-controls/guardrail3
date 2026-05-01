use g3ts_style_types::{G3TsStyleContractInput, G3TsStylePackageSurfaceState};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use std::collections::BTreeSet;

pub(crate) fn check_validate_runs_css_lint(
    contract: &G3TsStyleContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = package_rel_path(&contract.package);
    let Some(package) = parsed_package(&contract.package) else {
        results.push(error(
            "g3ts-style/validate-runs-css-lint",
            "Validate script cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `validate` runs CSS lint fail-closed.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
        return;
    };
    if validate_runs_css_lint(package) {
        results.push(info(
            "g3ts-style/validate-runs-css-lint",
            "Validate script runs CSS lint",
            format!(
                "`{}` defines a fail-closed `validate` script that reaches `lint:css` or direct `stylelint --max-warnings 0`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        ));
    } else {
        results.push(error(
            "g3ts-style/validate-runs-css-lint",
            "Validate script does not run CSS lint",
            format!(
                "`{}` must define a fail-closed `validate` script that invokes `lint:css` through a package-manager run command or directly invokes `stylelint --max-warnings 0`. This makes style validation part of the standard app quality gate.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
    }
}

fn validate_runs_css_lint(
    package: &g3ts_style_types::G3TsStylePackageSurfaceSnapshot,
) -> bool {
    if !package.script_names.iter().any(|name| name == "validate") {
        return false;
    }
    let reachable = reachable_script_names(package, "validate");
    if reachable.is_empty() {
        return false;
    }
    if package
        .script_parse_blockers
        .iter()
        .any(|blocker| reachable.contains(&blocker.script_name))
    {
        return false;
    }
    if package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && (invocation.preceded_by
                == Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or)
                || invocation.followed_by
                    == Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or))
    }) {
        return false;
    }

    package.script_tool_invocations.iter().any(|invocation| {
        reachable.contains(&invocation.script_name)
            && ((invocation.script_name == "lint:css"
                && stylelint_invocation_is_fail_closed(invocation))
                || (invocation.script_name == "validate"
                    && stylelint_invocation_is_fail_closed(invocation)))
    })
}

fn reachable_script_names(
    package: &g3ts_style_types::G3TsStylePackageSurfaceSnapshot,
    root_script_name: &str,
) -> BTreeSet<String> {
    if !package
        .script_names
        .iter()
        .any(|script_name| script_name == root_script_name)
    {
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

fn package_script_target(
    invocation: &g3ts_style_types::G3TsStylePackageScriptToolInvocation,
) -> Option<String> {
    if invocation.executable == "package-script" {
        return invocation
            .args
            .first()
            .map(|script_name| script_name.trim())
            .filter(|script_name| !script_name.is_empty())
            .map(str::to_owned);
    }

    if invocation.executable == "lint:css" {
        return Some("lint:css".to_owned());
    }

    if matches!(invocation.executable.as_str(), "pnpm" | "yarn" | "bun") {
        return invocation
            .args
            .first()
            .map(|script_name| script_name.trim())
            .filter(|script_name| *script_name == "lint:css")
            .map(str::to_owned);
    }

    None
}

fn stylelint_invocation_is_fail_closed(
    invocation: &g3ts_style_types::G3TsStylePackageScriptToolInvocation,
) -> bool {
    let Some(args) = stylelint_args(invocation) else {
        return false;
    };
    args.iter().any(|arg| arg == "--max-warnings")
        && args.iter().any(|arg| arg == "0")
        && invocation.preceded_by
            != Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or)
        && invocation.followed_by
            != Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or)
}

fn stylelint_args(
    invocation: &g3ts_style_types::G3TsStylePackageScriptToolInvocation,
) -> Option<&[String]> {
    if invocation.executable == "stylelint" {
        return Some(&invocation.args);
    }
    if matches!(invocation.executable.as_str(), "pnpm" | "npm" | "yarn" | "bun" | "npx" | "bunx")
    {
        let (tool, args) = invocation.args.split_first()?;
        if tool == "stylelint" {
            return Some(args);
        }
    }
    None
}

fn parsed_package(
    package: &G3TsStylePackageSurfaceState,
) -> Option<&g3ts_style_types::G3TsStylePackageSurfaceSnapshot> {
    match package {
        G3TsStylePackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsStylePackageSurfaceState::Missing { .. }
        | G3TsStylePackageSurfaceState::Unreadable { .. }
        | G3TsStylePackageSurfaceState::ParseError { .. } => None,
    }
}

fn package_rel_path(package: &G3TsStylePackageSurfaceState) -> Option<&str> {
    match package {
        G3TsStylePackageSurfaceState::Missing { rel_path }
        | G3TsStylePackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsStylePackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsStylePackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn info(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
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

fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}
