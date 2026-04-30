use g3ts_style_types::{
    G3TsStyleConfigChecksInput, G3TsStyleContractInput, G3TsStyleEslintSurfaceState,
    G3TsStylePackageSurfaceState, G3TsStylePolicySurfaceState, G3TsStylelintConfigSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const REQUIRED_PACKAGES: [&str; 5] = [
    "stylelint",
    "stylelint-config-standard",
    "stylelint-config-tailwindcss",
    "@double-great/stylelint-a11y",
    "eslint-plugin-tailwind-ban",
];
const REQUIRED_EXTENDS: [&str; 2] = ["stylelint-config-standard", "stylelint-config-tailwindcss"];
const REQUIRED_PLUGINS: [&str; 1] = ["@double-great/stylelint-a11y"];
const REQUIRED_A11Y_RULES: [&str; 11] = [
    "a11y/content-property-no-static-value",
    "a11y/font-size-is-readable",
    "a11y/line-height-is-vertical-rhythmed",
    "a11y/media-prefers-reduced-motion",
    "a11y/no-display-none",
    "a11y/no-obsolete-attribute",
    "a11y/no-obsolete-element",
    "a11y/no-outline-none",
    "a11y/no-spread-text",
    "a11y/no-text-align-justify",
    "a11y/selector-pseudo-class-focus",
];

#[must_use]
pub fn check(input: &G3TsStyleConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.contracts {
        check_policy(contract, &mut results);
        check_policy_paths(contract, &mut results);
        check_packages(contract, &mut results);
        check_stylelint(contract, &mut results);
        check_css_lint_script(contract, &mut results);
        check_tailwind_eslint(contract, &mut results);
    }
    results
}

fn check_policy(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = policy_rel_path(&contract.policy);
    match &contract.policy {
        G3TsStylePolicySurfaceState::Parsed { snapshot }
            if !snapshot.source_globs.is_empty() && !snapshot.stylelint_css_globs.is_empty() =>
        {
            results.push(info(
                "g3ts-style/strict-policy-configured",
                "Style policy is configured",
                format!("`{}` defines `[ts.style]` with source lanes and Stylelint CSS lanes.", rel_path.unwrap_or("guardrail3-ts.toml")),
                rel_path,
            ));
        }
        _ => results.push(error(
            "g3ts-style/strict-policy-configured",
            "Style policy is not configured",
            format!(
                "`{}` must define `[ts.style]` with non-empty `source_globs` and `stylelint_css_globs`.",
                rel_path.unwrap_or("guardrail3-ts.toml")
            ),
            rel_path,
        )),
    }
}

fn check_policy_paths(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = policy_rel_path(&contract.policy);
    let Some(snapshot) = parsed_policy(&contract.policy) else {
        return;
    };
    let invalid = invalid_policy_paths(snapshot);
    if invalid.is_empty() {
        results.push(info(
            "g3ts-style/policy-paths-valid",
            "Style policy paths are valid",
            format!(
                "`{}` uses app-relative `[ts.style]` source and CSS globs without parent traversal or external URLs.",
                rel_path.unwrap_or("guardrail3-ts.toml")
            ),
            rel_path,
        ));
    } else {
        results.push(error(
            "g3ts-style/policy-paths-valid",
            "Style policy paths are invalid",
            format!(
                "`{}` has invalid `[ts.style]` path values: {}. Values must be non-empty app-relative globs without `..` and must not be external URLs.",
                rel_path.unwrap_or("guardrail3-ts.toml"),
                invalid.join(", ")
            ),
            rel_path,
        ));
    }
}

fn check_packages(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let missing = REQUIRED_PACKAGES
        .iter()
        .filter(|package| !package_present(&contract.package, package))
        .collect::<Vec<_>>();
    let rel_path = package_rel_path(&contract.package);
    if missing.is_empty() {
        results.push(info(
            "g3ts-style/style-packages-present",
            "Style packages are installed",
            format!(
                "`{}` directly installs Stylelint, Tailwind style policy, and CSS a11y packages.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
    } else {
        results.push(error(
            "g3ts-style/style-packages-present",
            "Style packages are missing",
            format!(
                "`{}` must directly install these dependencies or devDependencies: {}.",
                rel_path.unwrap_or("package.json"),
                missing
                    .into_iter()
                    .map(|item| format!("`{item}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            rel_path,
        ));
    }
}

fn check_stylelint(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = stylelint_rel_path(&contract.stylelint_config);
    let Some(snapshot) = parsed_stylelint(&contract.stylelint_config) else {
        results.push(error(
            "g3ts-style/stylelint-config-present",
            "Stylelint config is missing",
            format!(
                "`{}` must exist and be readable.",
                rel_path.unwrap_or("stylelint.config.*")
            ),
            rel_path,
        ));
        return;
    };
    results.push(info(
        "g3ts-style/stylelint-config-present",
        "Stylelint config is present",
        format!("`{}` was resolved through Stylelint.", snapshot.rel_path),
        Some(&snapshot.rel_path),
    ));
    let stack_ok = REQUIRED_EXTENDS
        .iter()
        .all(|package| snapshot.raw_extends.iter().any(|value| value == package))
        && REQUIRED_PLUGINS
            .iter()
            .all(|package| snapshot.raw_plugins.iter().any(|value| value == package));
    if stack_ok {
        results.push(info(
            "g3ts-style/stylelint-config-stack",
            "Stylelint stack is configured",
            format!(
                "`{}` extends Stylelint standard and Tailwind configs and loads the a11y plugin.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    } else {
        results.push(error(
            "g3ts-style/stylelint-config-stack",
            "Stylelint stack is incomplete",
            format!(
                "`{}` must extend `stylelint-config-standard` and `stylelint-config-tailwindcss`, and load `@double-great/stylelint-a11y`.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    }
    let missing_rules = REQUIRED_A11Y_RULES
        .iter()
        .filter(|rule| {
            !snapshot
                .resolved_rule_names
                .iter()
                .any(|candidate| candidate == *rule)
        })
        .collect::<Vec<_>>();
    if snapshot.probe_present && !snapshot.probe_ignored && missing_rules.is_empty() {
        results.push(info(
            "g3ts-style/stylelint-a11y-rules",
            "Stylelint a11y rules are enabled",
            format!("`{}` enables the required CSS accessibility rules on `[ts.style].stylelint_css_globs`.", snapshot.rel_path),
            Some(&snapshot.rel_path),
        ));
    } else {
        results.push(error(
            "g3ts-style/stylelint-a11y-rules",
            "Stylelint a11y rules are not enabled",
            format!(
                "`{}` must enable these rules on `[ts.style].stylelint_css_globs`: {}.",
                snapshot.rel_path,
                missing_rules
                    .into_iter()
                    .map(|item| format!("`{item}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Some(&snapshot.rel_path),
        ));
    }
}

fn check_css_lint_script(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = package_rel_path(&contract.package);
    let Some(package) = parsed_package(&contract.package) else {
        return;
    };
    let has_blocker = package
        .script_parse_blockers
        .iter()
        .any(|blocker| blocker.script_name == "lint:css");
    let valid = !has_blocker
        && package.script_tool_invocations.iter().any(|invocation| {
            invocation.script_name == "lint:css"
                && invocation.executable == "stylelint"
                && invocation.args.iter().any(|arg| arg == "--max-warnings")
                && invocation.args.iter().any(|arg| arg == "0")
                && invocation.preceded_by
                    != Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or)
                && invocation.followed_by
                    != Some(g3ts_style_types::G3TsStylePackageScriptCommandSeparator::Or)
        });
    if valid {
        results.push(info(
            "g3ts-style/css-lint-script",
            "CSS lint script is fail-closed",
            format!(
                "`{}` defines `lint:css` with `stylelint --max-warnings 0`.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
    } else {
        results.push(error(
            "g3ts-style/css-lint-script",
            "CSS lint script is not fail-closed",
            format!(
                "`{}` must define `lint:css` that invokes `stylelint` with `--max-warnings 0` and does not use `||` fail-open separators.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        ));
    }
}

fn check_tailwind_eslint(contract: &G3TsStyleContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = eslint_rel_path(&contract.eslint_config);
    let Some(snapshot) = parsed_eslint(&contract.eslint_config) else {
        results.push(error(
            "g3ts-style/tailwind-ban-eslint-rule",
            "Tailwind ban ESLint rule is not effective",
            format!("`{}` must activate `tailwind-ban/no-deny-tailwind-tokens` at `error` with a non-empty ESLint-owned `denyList`.", rel_path.unwrap_or("eslint.config.*")),
            rel_path,
        ));
        return;
    };
    let plugin_ok = snapshot
        .source_plugin_package_names
        .get("tailwind-ban")
        .is_some_and(|packages| {
            packages
                .iter()
                .any(|package| package == "eslint-plugin-tailwind-ban")
        });
    if snapshot.source_probe_present
        && !snapshot.source_probe_ignored
        && plugin_ok
        && snapshot.tailwind_rule_effective
    {
        results.push(info(
            "g3ts-style/tailwind-ban-eslint-rule",
            "Tailwind ban ESLint rule is effective",
            format!("`{}` activates `tailwind-ban/no-deny-tailwind-tokens` at `error` with a non-empty ESLint-owned denyList.", snapshot.rel_path),
            Some(&snapshot.rel_path),
        ));
    } else {
        results.push(error(
            "g3ts-style/tailwind-ban-eslint-rule",
            "Tailwind ban ESLint rule is not effective",
            format!(
                "`{}` must activate plugin namespace `tailwind-ban` from `eslint-plugin-tailwind-ban` and rule `tailwind-ban/no-deny-tailwind-tokens` at `error` with a non-empty ESLint-owned `denyList` on every `[ts.style].source_globs` probe.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    }
}

fn valid_rel_path(path: &str) -> bool {
    !path.trim().is_empty()
        && !path.starts_with('/')
        && !path.contains("://")
        && !path.split('/').any(|segment| segment == "..")
}

fn invalid_policy_paths(
    snapshot: &g3ts_style_types::G3TsStylePolicySnapshot,
) -> Vec<String> {
    snapshot
        .source_globs
        .iter()
        .filter(|path| !valid_rel_path(path))
        .map(|path| format!("source_globs=`{path}`"))
        .chain(
            snapshot
                .stylelint_css_globs
                .iter()
                .filter(|path| !valid_rel_path(path))
                .map(|path| format!("stylelint_css_globs=`{path}`")),
        )
        .collect()
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

fn package_present(package: &G3TsStylePackageSurfaceState, dependency_name: &str) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

fn policy_rel_path(policy: &G3TsStylePolicySurfaceState) -> Option<&str> {
    match policy {
        G3TsStylePolicySurfaceState::Missing { rel_path }
        | G3TsStylePolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsStylePolicySurfaceState::ParseError { rel_path, .. }
        | G3TsStylePolicySurfaceState::MissingTsPolicy { rel_path }
        | G3TsStylePolicySurfaceState::MissingStylePolicy { rel_path } => Some(rel_path),
        G3TsStylePolicySurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn parsed_policy(
    policy: &G3TsStylePolicySurfaceState,
) -> Option<&g3ts_style_types::G3TsStylePolicySnapshot> {
    match policy {
        G3TsStylePolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsStylePolicySurfaceState::Missing { .. }
        | G3TsStylePolicySurfaceState::Unreadable { .. }
        | G3TsStylePolicySurfaceState::ParseError { .. }
        | G3TsStylePolicySurfaceState::MissingTsPolicy { .. }
        | G3TsStylePolicySurfaceState::MissingStylePolicy { .. } => None,
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

fn parsed_stylelint(
    config: &G3TsStylelintConfigSurfaceState,
) -> Option<&g3ts_style_types::G3TsStylelintConfigSnapshot> {
    match config {
        G3TsStylelintConfigSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsStylelintConfigSurfaceState::Missing { .. }
        | G3TsStylelintConfigSurfaceState::Unreadable { .. }
        | G3TsStylelintConfigSurfaceState::ParseError { .. } => None,
    }
}

fn stylelint_rel_path(config: &G3TsStylelintConfigSurfaceState) -> Option<&str> {
    match config {
        G3TsStylelintConfigSurfaceState::Missing { rel_path }
        | G3TsStylelintConfigSurfaceState::Unreadable { rel_path, .. }
        | G3TsStylelintConfigSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsStylelintConfigSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn parsed_eslint(
    config: &G3TsStyleEslintSurfaceState,
) -> Option<&g3ts_style_types::G3TsStyleEslintSurfaceSnapshot> {
    match config {
        G3TsStyleEslintSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsStyleEslintSurfaceState::Missing { .. }
        | G3TsStyleEslintSurfaceState::Unreadable { .. }
        | G3TsStyleEslintSurfaceState::ParseError { .. } => None,
    }
}

fn eslint_rel_path(config: &G3TsStyleEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsStyleEslintSurfaceState::Missing { rel_path }
        | G3TsStyleEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsStyleEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsStyleEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
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

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
