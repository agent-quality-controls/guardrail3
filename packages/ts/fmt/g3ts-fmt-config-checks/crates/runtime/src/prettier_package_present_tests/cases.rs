use g3ts_fmt_types::{
    G3TsFmtConfigChecksInput, G3TsFmtConfigSurfaceState, G3TsFmtContractInput,
    G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState, G3TsFmtSyncpackSurfaceState,
};

#[test]
fn missing_prettier_package_reports_package_rule() {
    let input = G3TsFmtConfigChecksInput {
        contracts: vec![G3TsFmtContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsFmtPackageSurfaceState::Parsed {
                snapshot: G3TsFmtPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: Vec::new(),
                    script_names: Vec::new(),
                    script_tool_invocations: Vec::new(),
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
    };

    g3ts_fmt_config_checks_assertions::prettier_package_present::assert_error(
        &input,
        Some("package.json"),
    );
}
