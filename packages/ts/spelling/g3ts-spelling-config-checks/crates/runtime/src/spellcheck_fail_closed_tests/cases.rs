use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageScriptCommandSeparator, G3TsSpellingPackageScriptToolInvocation,
    G3TsSpellingPackageSurfaceSnapshot, G3TsSpellingPackageSurfaceState,
    G3TsSpellingSyncpackSurfaceState,
};

#[test]
fn non_cspell_script_reports_fail_closed_rule() {
    let input = input(vec![invocation("spellcheck", "node", &["spellcheck.js"])]);

    g3ts_spelling_config_checks_assertions::spellcheck_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn bare_cspell_reports_fail_closed_rule() {
    let input = input(vec![invocation("spellcheck", "cspell", &[])]);

    g3ts_spelling_config_checks_assertions::spellcheck_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn cspell_with_flags_but_no_target_reports_fail_closed_rule() {
    let input = input(vec![invocation(
        "spellcheck",
        "cspell",
        &["--no-progress", "--no-summary"],
    )]);

    g3ts_spelling_config_checks_assertions::spellcheck_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn or_fallback_reports_fail_closed_rule() {
    let mut bad_invocation = invocation("spellcheck", "cspell", &["."]);
    bad_invocation.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let input = input(vec![
        invocation("spellcheck", "node", &["spellcheck.js"]),
        bad_invocation,
    ]);

    g3ts_spelling_config_checks_assertions::spellcheck_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn later_or_fallback_reports_fail_closed_rule() {
    let mut cspell = invocation("spellcheck", "cspell", &["."]);
    cspell.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::And);
    let mut echo = invocation("spellcheck", "echo", &["ok"]);
    echo.preceded_by = Some(G3TsSpellingPackageScriptCommandSeparator::And);
    echo.followed_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let mut fallback = invocation("spellcheck", "true", &[]);
    fallback.preceded_by = Some(G3TsSpellingPackageScriptCommandSeparator::Or);
    let input = input(vec![cspell, echo, fallback]);

    g3ts_spelling_config_checks_assertions::spellcheck_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

fn input(
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
                    script_names: vec!["spellcheck".to_owned()],
                    script_tool_invocations,
                    script_parse_blockers: Vec::new(),
                },
            },
            cspell_config: G3TsSpellingConfigSurfaceState::Missing {
                rel_path: "cspell.config.*".to_owned(),
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
