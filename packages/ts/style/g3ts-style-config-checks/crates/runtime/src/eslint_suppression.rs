use g3ts_style_types::{G3TsStyleContractInput, G3TsStyleEslintSurfaceState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn check_protected_style_rule_disables(
    contract: &G3TsStyleContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    const RESTRICT_RULE: &str = "@eslint-community/eslint-comments/no-restricted-disable";
    const PROTECTED_RULES: [&str; 2] = ["style-policy/*", "tailwind-ban/*"];

    let rel_path = eslint_rel_path(&contract.eslint_config);
    let Some(snapshot) = parsed_eslint(&contract.eslint_config) else {
        results.push(error(
            "g3ts-style/protected-style-rule-disables-restricted",
            "Style protected-disable policy cannot be checked",
            format!(
                "`{}` must configure `{RESTRICT_RULE}` for `style-policy/*` and `tailwind-ban/*` on every `[ts.style].source_globs` probe.",
                rel_path.unwrap_or("eslint.config.*")
            ),
            rel_path,
        ));
        return;
    };

    let restricted =
        !snapshot.source_probe_disable_policies.is_empty()
            && snapshot.source_probe_disable_policies.iter().all(|probe| {
                !probe.ignored
                    && probe
                        .warn_or_error_rules
                        .iter()
                        .any(|rule| rule == RESTRICT_RULE)
                    && PROTECTED_RULES.iter().all(|rule| {
                        probe
                            .restricted_disable_patterns
                            .iter()
                            .any(|pattern| pattern_covers_rule(pattern, rule))
                    })
            });

    if restricted {
        results.push(info(
            "g3ts-style/protected-style-rule-disables-restricted",
            "Style delegated-rule disables are restricted",
            format!(
                "`{}` enables `{RESTRICT_RULE}` and restricts disables for style-policy and legacy Tailwind-ban rules.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    } else {
        results.push(error(
            "g3ts-style/protected-style-rule-disables-restricted",
            "Style delegated-rule disables are not restricted",
            format!(
                "`{}` must enable `{RESTRICT_RULE}` at `warn` or `error` on `[ts.style].source_globs`, with options covering `style-policy/*` and `tailwind-ban/*`. This keeps agents from bypassing style architecture with `eslint-disable` comments.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    }
}

pub(crate) fn check_eslint_disable_inventory(
    directives: &[g3ts_style_types::G3TsStyleEslintDirectiveInput],
    results: &mut Vec<G3CheckResult>,
) {
    const ID: &str = "g3ts-style/eslint-disable-inventory";

    if directives.is_empty() {
        results.push(info(
            ID,
            "Style source contains no ESLint disable directives",
            "No ESLint disable directives were found in style source lanes.".to_owned(),
            Some("eslint.config.*"),
        ));
        return;
    }

    for directive in directives {
        if let Some(reason) = directive.parse_error.as_deref() {
            results.push(error(
                ID,
                "Style ESLint disable inventory cannot be parsed",
                format!(
                    "`{}` could not be parsed for ESLint disable directives: {reason}. G3TS fails closed because hidden disables would bypass delegated style rules.",
                    directive.rel_path
                ),
                Some(&directive.rel_path),
            ));
            continue;
        }

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "Style source contains an ESLint disable directive".to_owned(),
            format!(
                "`{}` line {} contains `{}` for {}. ESLint disables are allowed only as visible escape hatches; keep the directive described and avoid disabling protected style rules.",
                directive.rel_path,
                directive.line,
                directive.directive_kind,
                disabled_rules(directive),
            ),
            Some(directive.rel_path.clone()),
            None,
        ));
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

fn pattern_covers_rule(pattern: &str, rule: &str) -> bool {
    pattern == rule
        || pattern == "*"
        || pattern
            .strip_suffix('*')
            .is_some_and(|prefix| rule.starts_with(prefix))
}

fn disabled_rules(directive: &g3ts_style_types::G3TsStyleEslintDirectiveInput) -> String {
    if directive.all_rules {
        return "all rules".to_owned();
    }
    directive.disabled_rules.join(", ")
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
