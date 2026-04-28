use g3ts_astro_setup_config_checks_assertions::run as assertions;

#[test]
fn golden_setup_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-setup/astro-package-present",
            "g3ts-astro-setup/astro-check-present",
            "g3ts-astro-setup/astro-eslint-plugin-package-present",
            "g3ts-astro-setup/eslint-comments-plugin-package-present",
            "g3ts-astro-setup/lint-script",
            "g3ts-astro-setup/syncpack-lint-script",
            "g3ts-astro-setup/validate-script",
            "g3ts-astro-setup/astro-eslint-plugin-wired",
            "g3ts-astro-setup/eslint-disable-descriptions-required",
            "g3ts-astro-setup/unused-eslint-disables-fail",
            "g3ts-astro-setup/protected-setup-rule-disables-restricted",
            "g3ts-astro-setup/syncpack-stack-pins",
            "g3ts-astro-setup/syncpack-forbidden-deps",
            "g3ts-astro-setup/site-url",
            "g3ts-astro-setup/static-output",
            "g3ts-astro-setup/required-integrations",
        ],
    );
}

#[test]
fn eslint_comments_plugin_package_must_be_installed() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package
        .dev_dependencies
        .retain(|name| name != "@eslint-community/eslint-plugin-eslint-comments");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/eslint-comments-plugin-package-present",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn eslint_disable_descriptions_must_be_error_on_source_lanes() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot
        .tsx_source_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/require-description");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn unused_eslint_disables_must_fail_closed_on_source_lanes() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot.ts_source_unused_disable_fail_closed = false;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/unused-eslint-disables-fail",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn eslint_disable_descriptions_accepts_namespace_when_package_identity_is_unavailable() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot.ts_source_plugin_package_names.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn eslint_disable_descriptions_rejects_missing_plugin_namespace() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot
        .ts_source_plugins
        .retain(|plugin| plugin != "@eslint-community/eslint-comments");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_setup_rule_disables_must_cover_astro_valid_compile() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot
        .astro_source_restricted_disable_patterns
        .retain(|rule| rule != "astro/valid-compile");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/protected-setup-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_setup_rule_disables_requires_restrict_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot
        .ts_source_warn_or_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/no-restricted-disable");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/protected-setup-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_script_must_exist() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_names.retain(|name| name != "validate");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_script_must_not_have_parse_blockers() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_parse_blockers.push(
        g3ts_astro_setup_types::G3TsAstroPackageScriptParseBlocker {
            script_name: "validate".to_owned(),
            reason: "unsupported shell construct".to_owned(),
        },
    );

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn unrelated_start_script_parse_blocker_does_not_break_validation_tools() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_names.push("start".to_owned());
    package.script_bodies.push((
        "start".to_owned(),
        "astro preview --port ${PORT:-3001}".to_owned(),
    ));
    package.script_parse_blockers.push(
        g3ts_astro_setup_types::G3TsAstroPackageScriptParseBlocker {
            script_name: "start".to_owned(),
            reason: "script command contains invalid shell syntax".to_owned(),
        },
    );

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/astro-check-present",
        guardrail3_check_types::G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/lint-script",
        guardrail3_check_types::G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/syncpack-lint-script",
        guardrail3_check_types::G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn validate_script_must_reach_astro_build() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_tool_invocations.retain(|invocation| {
        !(invocation.script_name == "build" && invocation.executable == "astro")
    });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_script_must_reach_eslint() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package
        .script_tool_invocations
        .retain(|invocation| invocation.executable != "eslint");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_script_must_reach_syncpack_lint() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_tool_invocations.retain(|invocation| {
        !(invocation.executable == "syncpack"
            && invocation.args.first().is_some_and(|arg| arg == "lint"))
    });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_script_must_reach_astro_check() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_tool_invocations.retain(|invocation| {
        !(invocation.executable == "astro"
            && invocation.args.first().is_some_and(|arg| arg == "check"))
    });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validate_lifecycle_scripts_must_be_safe() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package.script_names.push("prevalidate".to_owned());
    package
        .script_bodies
        .push(("prevalidate".to_owned(), "eslint . || true".to_owned()));
    package.script_tool_invocations.push(
        g3ts_astro_setup_types::G3TsAstroPackageScriptToolInvocation {
            script_name: "prevalidate".to_owned(),
            command_index: 0,
            invocation: "eslint .".to_owned(),
            executable: "eslint".to_owned(),
            args: vec![".".to_owned()],
            preceded_by: None,
            followed_by: Some(g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or),
        },
    );

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validation_like_sibling_scripts_must_all_be_safe() {
    for script_name in ["postvalidate", "verify", "ci", "precommit", "lint:all"] {
        let mut input = super::helpers::golden();
        let package = super::helpers::parsed_package_mut(&mut input);
        package.script_names.push(script_name.to_owned());
        package
            .script_bodies
            .push((script_name.to_owned(), "eslint . || true".to_owned()));
        package.script_tool_invocations.push(
            g3ts_astro_setup_types::G3TsAstroPackageScriptToolInvocation {
                script_name: script_name.to_owned(),
                command_index: 0,
                invocation: "eslint .".to_owned(),
                executable: "eslint".to_owned(),
                args: vec![".".to_owned()],
                preceded_by: None,
                followed_by: Some(
                    g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or,
                ),
            },
        );

        assertions::assert_runtime_check_id_severity(
            &input,
            "g3ts-astro-setup/validate-script",
            guardrail3_check_types::G3Severity::Error,
        );
    }
}

#[test]
fn validate_script_must_not_hide_failures() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    for invocation in &mut package.script_tool_invocations {
        if invocation.script_name == "validate" && invocation.command_index == 0 {
            invocation.followed_by =
                Some(g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or);
        }
    }

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn validation_like_sibling_scripts_must_be_safe() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    for invocation in &mut package.script_tool_invocations {
        if invocation.script_name == "check" {
            invocation.followed_by =
                Some(g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::Or);
        }
    }

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-setup/validate-script",
        guardrail3_check_types::G3Severity::Error,
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
