use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageScriptCommandSeparator, G3TsSpellingPackageScriptToolInvocation,
    G3TsSpellingPackageSurfaceSnapshot, G3TsSpellingPackageSurfaceState,
    G3TsSpellingSyncpackSurfaceState,
};

#[test]
fn missing_validate_reports_validate_rule() {
    let input = input(
        vec!["spellcheck"],
        vec![invocation("spellcheck", "cspell", &["."])],
    );

    g3ts_spelling_config_checks_assertions::validate_runs_spellcheck::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn validate_or_fallback_reports_validate_rule() {
    let mut validate = invocation("validate", "package-script", &["spellcheck"]);
    validate.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let input = input(
        vec!["validate", "spellcheck"],
        vec![validate, invocation("spellcheck", "cspell", &["."])],
    );

    g3ts_spelling_config_checks_assertions::validate_runs_spellcheck::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn validate_to_spellcheck_to_cspell_is_accepted() {
    let input = input(
        vec!["validate", "spellcheck"],
        vec![
            invocation("validate", "package-script", &["spellcheck"]),
            invocation("spellcheck", "cspell", &["."]),
        ],
    );

    g3ts_spelling_config_checks_assertions::validate_runs_spellcheck::assert_info(
        &input,
        Some("package.json"),
    );
}

#[test]
fn downstream_spellcheck_fallback_reports_validate_rule() {
    let mut cspell = invocation("spellcheck", "cspell", &["."]);
    cspell.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::And);
    let mut echo = invocation("spellcheck", "echo", &["ok"]);
    echo.preceded_by = Some(G3TsSpellingPackageScriptCommandSeparator::And);
    echo.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let mut fallback = invocation("spellcheck", "true", &[]);
    fallback.preceded_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let input = input(
        vec!["validate", "spellcheck"],
        vec![
            invocation("validate", "package-script", &["spellcheck"]),
            cspell,
            echo,
            fallback,
        ],
    );

    g3ts_spelling_config_checks_assertions::validate_runs_spellcheck::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn direct_validate_cspell_is_accepted() {
    let input = input(
        vec!["validate"],
        vec![invocation("validate", "cspell", &["."])],
    );

    g3ts_spelling_config_checks_assertions::validate_runs_spellcheck::assert_info(
        &input,
        Some("package.json"),
    );
}

fn input(
    script_names: Vec<&str>,
    script_tool_invocations: Vec<G3TsSpellingPackageScriptToolInvocation>,
) -> G3TsSpellingConfigChecksInput {
    G3TsSpellingConfigChecksInput {
        contracts: vec![G3TsSpellingContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsSpellingPackageSurfaceState::Parsed {
                snapshot: G3TsSpellingPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["cspell".to_owned()],
                    script_names: script_names.into_iter().map(str::to_owned).collect(),
                    script_tool_invocations,
                    script_parse_blockers: Vec::new(),
                },
            },
            cspell_config: G3TsSpellingConfigSurfaceState::Parsed {
                rel_path: "cspell.json".to_owned(),
            },
            syncpack_config: G3TsSpellingSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}

fn invocation(
    script_name: &str,
    executable: &str,
    args: &[&str],
) -> G3TsSpellingPackageScriptToolInvocation {
    G3TsSpellingPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        executable: executable.to_owned(),
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        preceded_by: None,
        followed_by: None,
    }
}
