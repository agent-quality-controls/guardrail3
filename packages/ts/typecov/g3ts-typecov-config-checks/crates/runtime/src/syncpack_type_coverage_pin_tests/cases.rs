use g3ts_typecov_types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput, G3TsTypecovPackageSurfaceSnapshot,
    G3TsTypecovPackageSurfaceState, G3TsTypecovSyncpackSnapshot, G3TsTypecovSyncpackSurfaceState,
    G3TsTypecovSyncpackVersionGroupSnapshot,
};

#[test]
fn ignored_type_coverage_group_reports_syncpack_rule() {
    let input = input(vec![group(Some(true), Some("8.20.0"))]);

    g3ts_typecov_config_checks_assertions::syncpack_type_coverage_pin::assert_error(
        &input,
        Some(".syncpackrc"),
    );
}

#[test]
fn missing_pin_version_reports_syncpack_rule() {
    let input = input(vec![group(None, None)]);

    g3ts_typecov_config_checks_assertions::syncpack_type_coverage_pin::assert_error(
        &input,
        Some(".syncpackrc"),
    );
}

#[test]
fn banned_type_coverage_group_reports_syncpack_rule() {
    let input = input(vec![banned_group()]);

    g3ts_typecov_config_checks_assertions::syncpack_type_coverage_pin::assert_error(
        &input,
        Some(".syncpackrc"),
    );
}

#[test]
fn non_ignored_type_coverage_pin_is_accepted() {
    let input = input(vec![group(None, Some("8.20.0"))]);

    g3ts_typecov_config_checks_assertions::syncpack_type_coverage_pin::assert_info(
        &input,
        Some(".syncpackrc"),
    );
}

fn input(
    version_groups: Vec<G3TsTypecovSyncpackVersionGroupSnapshot>,
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
                    script_tool_invocations: Vec::new(),
                    script_parse_blockers: Vec::new(),
                },
            },
            syncpack_config: G3TsTypecovSyncpackSurfaceState::Parsed {
                snapshot: G3TsTypecovSyncpackSnapshot {
                    rel_path: ".syncpackrc".to_owned(),
                    source: Vec::new(),
                    version_groups,
                },
            },
        }],
    }
}

fn group(
    is_ignored: Option<bool>,
    pin_version: Option<&str>,
) -> G3TsTypecovSyncpackVersionGroupSnapshot {
    G3TsTypecovSyncpackVersionGroupSnapshot {
        dependencies: vec!["type-coverage".to_owned()],
        dependency_types: Vec::new(),
        packages: None,
        specifier_types: None,
        is_ignored,
        is_banned: None,
        pin_version: pin_version.map(str::to_owned),
    }
}

fn banned_group() -> G3TsTypecovSyncpackVersionGroupSnapshot {
    G3TsTypecovSyncpackVersionGroupSnapshot {
        dependencies: vec!["type-coverage".to_owned()],
        dependency_types: Vec::new(),
        packages: None,
        specifier_types: None,
        is_ignored: None,
        is_banned: Some(true),
        pin_version: Some("8.20.0".to_owned()),
    }
}
