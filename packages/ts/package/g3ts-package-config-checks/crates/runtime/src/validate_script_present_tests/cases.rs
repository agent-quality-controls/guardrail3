use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageRootSnapshot, G3TsPackageRootState,
    G3TsPackageSyncpackConfigState,
};

#[test]
fn missing_validate_script_reports_validate_presence_rule() {
    g3ts_package_config_checks_assertions::validate_script_present::assert_missing_validate_error_for_input(&input(None));
}

fn input(validate_script: Option<&str>) -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: G3TsPackageRootSnapshot {
                rel_path: "package.json".to_owned(),
                private_field: Some(true),
                package_manager: Some("pnpm@10.32.0".to_owned()),
                engines_node: Some(">=24".to_owned()),
                engines_pnpm: Some("10".to_owned()),
                preinstall_script: Some("npx only-allow pnpm".to_owned()),
                prepare_script: Some("echo prepare".to_owned()),
                lint_script: Some("eslint .".to_owned()),
                typecheck_script: Some("tsc --noEmit".to_owned()),
                validate_script: validate_script.map(str::to_owned),
                dependencies: Vec::new(),
                dev_dependencies: Vec::new(),
                pnpm_override_keys: Vec::new(),
                pnpm_only_built_dependencies: Vec::new(),
                script_commands: Vec::new(),
                script_tool_invocations: Vec::new(),
                script_parse_blockers: Vec::new(),
                safely_runs_only_allow_pnpm: true,
                safely_runs_syncpack_lint: false,
            },
        },
        locals: Vec::new(),
        syncpack_config: G3TsPackageSyncpackConfigState::Missing {
            rel_path: ".syncpackrc".to_owned(),
        },
        forbidden_syncpack_deps: Vec::new(),
    }
}
