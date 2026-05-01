use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageRootSnapshot, G3TsPackageRootState,
    G3TsPackageScriptCommandSeparator, G3TsPackageScriptToolInvocation,
    G3TsPackageSyncpackConfigState,
};

#[test]
fn validate_script_with_later_or_reports_fail_closed_rule() {
    let mut lint = invocation("pnpm lint", "package-script", &["lint"]);
    lint.followed_by = Some(G3TsPackageScriptCommandSeparator::And);
    let mut typecheck = invocation("pnpm typecheck", "package-script", &["typecheck"]);
    typecheck.preceded_by = Some(G3TsPackageScriptCommandSeparator::And);
    typecheck.followed_by = Some(G3TsPackageScriptCommandSeparator::Or);
    let mut fallback = invocation("true", "true", &[]);
    fallback.preceded_by = Some(G3TsPackageScriptCommandSeparator::Or);

    g3ts_package_config_checks_assertions::validate_script_fail_closed::assert_fail_open_validate_error_for_input(&input(vec![lint, typecheck, fallback]));
}

#[test]
fn validate_script_with_supported_chain_is_accepted() {
    let mut lint = invocation("pnpm lint", "package-script", &["lint"]);
    lint.followed_by = Some(G3TsPackageScriptCommandSeparator::And);
    let mut typecheck = invocation("pnpm typecheck", "package-script", &["typecheck"]);
    typecheck.preceded_by = Some(G3TsPackageScriptCommandSeparator::And);

    g3ts_package_config_checks_assertions::validate_script_fail_closed::assert_fail_closed_validate_inventory_for_input(&input(vec![lint, typecheck]));
}

fn input(script_tool_invocations: Vec<G3TsPackageScriptToolInvocation>) -> G3TsPackageChecksInput {
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
                validate_script: Some("pnpm lint && pnpm typecheck".to_owned()),
                dependencies: Vec::new(),
                dev_dependencies: Vec::new(),
                pnpm_override_keys: Vec::new(),
                pnpm_only_built_dependencies: Vec::new(),
                script_commands: Vec::new(),
                script_tool_invocations,
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

fn invocation(
    invocation: &str,
    executable: &str,
    args: &[&str],
) -> G3TsPackageScriptToolInvocation {
    G3TsPackageScriptToolInvocation {
        script_name: "validate".to_owned(),
        command_index: 0,
        invocation: invocation.to_owned(),
        executable: executable.to_owned(),
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        preceded_by: None,
        followed_by: None,
    }
}
