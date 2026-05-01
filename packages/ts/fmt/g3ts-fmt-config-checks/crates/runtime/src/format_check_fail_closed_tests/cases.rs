use g3ts_fmt_types::{
    G3TsFmtConfigChecksInput, G3TsFmtConfigSurfaceState, G3TsFmtContractInput,
    G3TsFmtPackageScriptCommandSeparator, G3TsFmtPackageScriptToolInvocation,
    G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState, G3TsFmtSyncpackSurfaceState,
};

#[test]
fn missing_check_flag_reports_fail_closed_rule() {
    let input = input(vec![
        invocation("format", "prettier", &["--write", "."]),
        invocation("format:check", "prettier", &["."]),
    ]);

    g3ts_fmt_config_checks_assertions::format_check_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn or_fallback_reports_fail_closed_rule() {
    let mut bad_invocation = invocation("format:check", "prettier", &["--check", "."]);
    bad_invocation.followed_by = Some(G3TsFmtPackageScriptCommandSeparator::Or);
    let input = input(vec![
        invocation("format", "prettier", &["--write", "."]),
        bad_invocation,
    ]);

    g3ts_fmt_config_checks_assertions::format_check_fail_closed::assert_error(
        &input,
        Some("package.json"),
    );
}

fn input(
    script_tool_invocations: Vec<G3TsFmtPackageScriptToolInvocation>,
) -> G3TsFmtConfigChecksInput {
    G3TsFmtConfigChecksInput {
        contracts: vec![G3TsFmtContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsFmtPackageSurfaceState::Parsed {
                snapshot: G3TsFmtPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["prettier".to_owned()],
                    script_names: vec!["format".to_owned(), "format:check".to_owned()],
                    script_tool_invocations,
                    script_parse_blockers: Vec::new(),
                },
            },
            prettier_config: G3TsFmtConfigSurfaceState::Missing {
                rel_path: "prettier.config.*".to_owned(),
            },
            syncpack_config: G3TsFmtSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}

fn invocation(
    script_name: &str,
    executable: &str,
    args: &[&str],
) -> G3TsFmtPackageScriptToolInvocation {
    G3TsFmtPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        executable: executable.to_owned(),
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        preceded_by: None,
        followed_by: None,
    }
}
