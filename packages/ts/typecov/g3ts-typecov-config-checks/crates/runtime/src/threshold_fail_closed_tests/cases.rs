use g3ts_typecov_types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput,
    G3TsTypecovPackageScriptCommandSeparator, G3TsTypecovPackageScriptToolInvocation,
    G3TsTypecovPackageSurfaceSnapshot, G3TsTypecovPackageSurfaceState,
    G3TsTypecovSyncpackSurfaceState,
};

#[test]
fn non_type_coverage_script_reports_fail_closed_rule() {
    let input = input(vec![invocation("typecov", "node", &["typecov.js"])]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn or_fallback_reports_fail_closed_rule() {
    let mut bad_invocation = invocation("typecov", "type-coverage", &["--at-least", "100"]);
    bad_invocation.followed_by = Some(G3TsTypecovPackageScriptCommandSeparator::Or);
    let input = input(vec![
        invocation("typecov", "node", &["typecov.js"]),
        bad_invocation,
    ]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn later_or_fallback_reports_fail_closed_rule() {
    let mut type_coverage = invocation("typecov", "type-coverage", &["--at-least", "100"]);
    type_coverage.followed_by = Some(G3TsTypecovPackageScriptCommandSeparator::And);
    let mut echo = invocation("typecov", "echo", &["ok"]);
    echo.preceded_by = Some(G3TsTypecovPackageScriptCommandSeparator::And);
    echo.followed_by = Some(G3TsTypecovPackageScriptCommandSeparator::Or);
    let mut fallback = invocation("typecov", "true", &[]);
    fallback.preceded_by = Some(G3TsTypecovPackageScriptCommandSeparator::Or);
    let input = input(vec![type_coverage, echo, fallback]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn exact_threshold_is_accepted() {
    let input = input(vec![invocation(
        "typecov",
        "type-coverage",
        &["--at-least", "100"],
    )]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_info(
        &input,
        Some("package.json"),
    );
}

#[test]
fn missing_threshold_reports_fail_closed_rule() {
    let input = input(vec![invocation("typecov", "type-coverage", &[])]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn weaker_threshold_reports_fail_closed_rule() {
    let input = input(vec![invocation(
        "typecov",
        "type-coverage",
        &["--at-least", "99"],
    )]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn equals_threshold_syntax_reports_fail_closed_rule() {
    let input = input(vec![invocation(
        "typecov",
        "type-coverage",
        &["--at-least=100"],
    )]);

    g3ts_typecov_config_checks_assertions::threshold_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

fn input(
    script_tool_invocations: Vec<G3TsTypecovPackageScriptToolInvocation>,
) -> G3TsTypecovConfigChecksInput {
    G3TsTypecovConfigChecksInput {
        contracts: vec![G3TsTypecovContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsTypecovPackageSurfaceState::Parsed {
                snapshot: G3TsTypecovPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["type-coverage".to_owned()],
                    script_names: vec!["typecov".to_owned()],
                    script_tool_invocations,
                    script_parse_blockers: Vec::new(),
                },
            },
            syncpack_config: G3TsTypecovSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}

fn invocation(
    script_name: &str,
    executable: &str,
    args: &[&str],
) -> G3TsTypecovPackageScriptToolInvocation {
    G3TsTypecovPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        executable: executable.to_owned(),
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        preceded_by: None,
        followed_by: None,
    }
}
