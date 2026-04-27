use g3ts_astro_setup_config_checks_assertions::run as assertions;

#[test]
fn golden_setup_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-setup/astro-package-present",
            "g3ts-astro-setup/astro-check-present",
            "g3ts-astro-setup/astro-eslint-plugin-package-present",
            "g3ts-astro-setup/lint-script",
            "g3ts-astro-setup/syncpack-lint-script",
            "g3ts-astro-setup/astro-eslint-plugin-wired",
            "g3ts-astro-setup/syncpack-stack-pins",
            "g3ts-astro-setup/syncpack-forbidden-deps",
            "g3ts-astro-setup/site-url",
            "g3ts-astro-setup/static-output",
            "g3ts-astro-setup/required-integrations",
        ],
    );
}

#[test]
fn astro_plugin_wiring_rejects_missing_effective_package_identity() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot.astro_source_plugin_package_names.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/astro-eslint-plugin-wired",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn lint_script_must_run_eslint_fail_closed() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_names.retain(|name| name != "lint");
    package.script_bodies.retain(|(name, _body)| name != "lint");
    package
        .script_commands
        .retain(|command| command.script_name != "lint");
    package
        .script_tool_invocations
        .retain(|invocation| invocation.script_name != "lint");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/lint-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn lint_script_must_not_hide_eslint_failure() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    for invocation in &mut package.script_tool_invocations {
        if invocation.script_name == "lint" {
            invocation.followed_by =
                Some(g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or);
        }
    }

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/lint-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn syncpack_lint_script_must_run_syncpack_lint_fail_closed() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_names.retain(|name| name != "lint:packages");
    package
        .script_bodies
        .retain(|(name, _body)| name != "lint:packages");
    package
        .script_commands
        .retain(|command| command.script_name != "lint:packages");
    package
        .script_tool_invocations
        .retain(|invocation| invocation.script_name != "lint:packages");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/syncpack-lint-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn syncpack_lint_script_must_not_hide_syncpack_failure() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    for invocation in &mut package.script_tool_invocations {
        if invocation.script_name == "lint:packages" {
            invocation.followed_by =
                Some(g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or);
        }
    }

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/syncpack-lint-script",
        guardrail3_check_types::G3Severity::Error,
    );
}
