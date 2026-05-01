use g3ts_typecov_types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput, G3TsTypecovPackageScriptParseBlocker,
    G3TsTypecovPackageSurfaceSnapshot, G3TsTypecovPackageSurfaceState,
    G3TsTypecovSyncpackSurfaceState,
};

#[test]
fn unparseable_typecov_reports_script_rule() {
    let input = input(vec!["typecov"], vec!["typecov"]);

    g3ts_typecov_config_checks_assertions::script_present::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn missing_typecov_reports_script_rule() {
    let input = input(vec!["validate"], Vec::new());

    g3ts_typecov_config_checks_assertions::script_present::assert_error(
        &input,
        Some("package.json"),
    );
}

fn input(script_names: Vec<&str>, blocked_scripts: Vec<&str>) -> G3TsTypecovConfigChecksInput {
    G3TsTypecovConfigChecksInput {
        contracts: vec![G3TsTypecovContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsTypecovPackageSurfaceState::Parsed {
                snapshot: G3TsTypecovPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["type-coverage".to_owned()],
                    script_names: script_names.into_iter().map(str::to_owned).collect(),
                    script_tool_invocations: Vec::new(),
                    script_parse_blockers: blocked_scripts
                        .into_iter()
                        .map(|script_name| G3TsTypecovPackageScriptParseBlocker {
                            script_name: script_name.to_owned(),
                            reason: "unsupported shell syntax".to_owned(),
                        })
                        .collect(),
                },
            },
            syncpack_config: G3TsTypecovSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}
