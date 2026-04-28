use g3ts_astro_seo_config_checks_assertions::run as assertions;

#[test]
fn golden_seo_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-seo/canonical-site-config",
            "g3ts-astro-seo/static-output-config",
            "g3ts-astro-seo/trailing-slash-policy",
            "g3ts-astro-seo/nuasite-checks",
            "g3ts-astro-seo/sitemap-integration-present",
            "g3ts-astro-seo/robots-integration-present",
            "g3ts-astro-seo/sitemap-generator-package-present",
            "g3ts-astro-seo/robots-generator-package-present",
            "g3ts-astro-seo/sitemap-checks-package-present",
            "g3ts-astro-seo/robots-checks-package-present",
            "g3ts-astro-seo/sitemap-checks-validate-script",
            "g3ts-astro-seo/robots-checks-validate-script",
            "g3ts-astro-seo/broad-crawler-generator-absent",
            "g3ts-astro-seo/seo-packages",
            "g3ts-astro-seo/structured-data-check",
            "g3ts-astro-seo/strict-policy-paths",
            "g3ts-astro-seo/policy-helper-surfaces",
            "g3ts-astro-seo/metadata-helper-rule",
            "g3ts-astro-seo/json-ld-helper-rule",
            "g3ts-astro-seo/protected-seo-rule-disables-restricted",
            "g3ts-astro-seo/eslint-disable-inventory",
        ],
    );
}

#[test]
fn protected_seo_rule_disables_must_cover_json_ld_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden seo eslint config should be parsed");
    };
    snapshot
        .tsx_source_restricted_disable_patterns
        .retain(|rule| rule != "astro-pipeline/require-approved-json-ld-helper-in-routes");

    assertions::assert_runtime_error_id(
        &input,
        "g3ts-astro-seo/protected-seo-rule-disables-restricted",
    );
}

#[test]
fn protected_seo_rule_disables_requires_restrict_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden seo eslint config should be parsed");
    };
    snapshot
        .ts_source_warn_or_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/no-restricted-disable");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-seo/protected-seo-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_seo_rule_disables_accept_pipeline_wildcard() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden seo eslint config should be parsed");
    };
    snapshot.astro_source_restricted_disable_patterns = vec!["astro-pipeline/*".to_owned()];
    snapshot.ts_source_restricted_disable_patterns = vec!["astro-pipeline/*".to_owned()];
    snapshot.tsx_source_restricted_disable_patterns = vec!["astro-pipeline/*".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-seo/protected-seo-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn eslint_disable_inventory_warns_when_seo_source_contains_disable() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput {
            rel_path: "src/pages/index.astro".to_owned(),
            directive_kind: "DisableNextLine".to_owned(),
            disabled_rules: vec![
                "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
            ],
            all_rules: false,
            line: 20,
            target_line: Some(21),
            parse_error: None,
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-seo/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Warn,
    );
}

#[test]
fn eslint_disable_inventory_fails_closed_on_seo_parse_error() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput {
            rel_path: "src/pages/index.astro".to_owned(),
            directive_kind: "ParseError".to_owned(),
            disabled_rules: Vec::new(),
            all_rules: false,
            line: 0,
            target_line: None,
            parse_error: Some("ambiguous directive syntax".to_owned()),
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-seo/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_site_fails_canonical_site_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.site = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/canonical-site-config");
}

#[test]
fn http_site_fails_canonical_site_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.site = Some("http://example.com".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/canonical-site-config");
}

#[test]
fn missing_static_output_fails_static_output_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.output = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/static-output-config");
}

#[test]
fn missing_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.trailing_slash = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn ignored_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.trailing_slash = Some(g3ts_astro_seo_types::G3TsAstroTrailingSlashPolicy::Ignore);
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn missing_sitemap_checker_package_fails() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "g3ts-astro-sitemap-checks");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-package-present");
}

#[test]
fn missing_sitemap_generator_package_fails_integration_rule() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "@astrojs/sitemap");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-generator-package-present");
}

#[test]
fn missing_sitemap_validate_script_call_fails() {
    let mut input = super::helpers::golden();
    remove_tool_invocation(&mut input, "g3ts-astro-sitemap-checks");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-validate-script");
}

#[test]
fn malformed_sitemap_validate_script_args_fail() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        for invocation in &mut snapshot.script_tool_invocations {
            if invocation.executable == "g3ts-astro-sitemap-checks" {
                invocation.args = vec!["--output-dir".to_owned(), "tmp".to_owned()];
            }
        }
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-validate-script");
}

#[test]
fn validate_script_uses_configured_astro_output_dir() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.out_dir = Some("public-build".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-validate-script");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-checks-validate-script");
}

#[test]
fn validate_script_accepts_configured_astro_output_dir() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.out_dir = Some("public-build".to_owned());
    }
    rewrite_validate_output_dir(&mut input, "public-build");

    assertions::assert_runtime_check_exact_ids(&input, golden_expected_ids());
}

#[test]
fn missing_sitemap_integration_fails() {
    let mut input = super::helpers::golden();
    remove_integration(&mut input, "@astrojs/sitemap");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-integration-present");
}

#[test]
fn missing_robots_checker_package_fails() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "g3ts-astro-robots-checks");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-checks-package-present");
}

#[test]
fn missing_robots_generator_package_fails_integration_rule() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "astro-robots");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-generator-package-present");
}

#[test]
fn missing_robots_integration_fails() {
    let mut input = super::helpers::golden();
    remove_integration(&mut input, "astro-robots");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-integration-present");
}

#[test]
fn missing_robots_validate_script_call_fails() {
    let mut input = super::helpers::golden();
    remove_tool_invocation(&mut input, "g3ts-astro-robots-checks");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-checks-validate-script");
}

#[test]
fn checker_before_build_fails_validate_script_order() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        for invocation in &mut snapshot.script_tool_invocations {
            if invocation.executable == "astro" {
                invocation.command_index = 2;
            }
            if invocation.executable == "g3ts-astro-sitemap-checks" {
                invocation.command_index = 0;
            }
        }
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-validate-script");
}

#[test]
fn never_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot.trailing_slash = Some(g3ts_astro_seo_types::G3TsAstroTrailingSlashPolicy::Never);
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn strict_ai_readable_requires_llms_package_script_and_integration() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    {
        snapshot.strict_ai_readable = true;
        snapshot.llms_required_sections = vec!["Docs".to_owned()];
        snapshot.llms_required_links = vec!["https://example.com/docs/".to_owned()];
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-integration-present");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-generator-package-present");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-checks-package-present");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-checks-validate-script");
}

#[test]
fn strict_ai_readable_passes_when_llms_package_integration_and_script_are_present() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    {
        snapshot.strict_ai_readable = true;
    }
    add_dev_dependency(&mut input, "g3ts-astro-llms");
    add_dev_dependency(&mut input, "g3ts-astro-llms-checks");
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms".to_owned()),
                name: Some("g3ts-astro-llms".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(llms_options()),
                }),
            });
    }
    add_tool_invocation(
        &mut input,
        "g3ts-astro-llms-checks",
        &[
            "--output-dir",
            "dist",
            "--required-section",
            "Docs",
            "--required-link",
            "https://example.com/docs/",
        ],
        3,
    );

    assertions::assert_runtime_check_exact_ids(
        &input,
        &[
            "g3ts-astro-seo/canonical-site-config",
            "g3ts-astro-seo/static-output-config",
            "g3ts-astro-seo/trailing-slash-policy",
            "g3ts-astro-seo/nuasite-checks",
            "g3ts-astro-seo/sitemap-integration-present",
            "g3ts-astro-seo/robots-integration-present",
            "g3ts-astro-seo/llms-integration-present",
            "g3ts-astro-seo/sitemap-generator-package-present",
            "g3ts-astro-seo/robots-generator-package-present",
            "g3ts-astro-seo/sitemap-checks-package-present",
            "g3ts-astro-seo/robots-checks-package-present",
            "g3ts-astro-seo/llms-generator-package-present",
            "g3ts-astro-seo/llms-checks-package-present",
            "g3ts-astro-seo/sitemap-checks-validate-script",
            "g3ts-astro-seo/robots-checks-validate-script",
            "g3ts-astro-seo/llms-checks-validate-script",
            "g3ts-astro-seo/broad-crawler-generator-absent",
            "g3ts-astro-seo/seo-packages",
            "g3ts-astro-seo/structured-data-check",
            "g3ts-astro-seo/strict-policy-paths",
            "g3ts-astro-seo/policy-helper-surfaces",
            "g3ts-astro-seo/metadata-helper-rule",
            "g3ts-astro-seo/json-ld-helper-rule",
            "g3ts-astro-seo/protected-seo-rule-disables-restricted",
            "g3ts-astro-seo/eslint-disable-inventory",
        ],
    );
}

#[test]
fn validate_script_accepts_safe_nested_package_scripts() {
    let mut input = super::helpers::golden();
    replace_validate_with_nested_scripts(&mut input, false);

    assertions::assert_runtime_check_exact_ids(&input, golden_expected_ids());
}

#[test]
fn validate_script_accepts_child_script_that_builds_then_checks_artifacts() {
    let mut input = super::helpers::golden();
    replace_validate_with_single_nested_artifact_script(&mut input);

    assertions::assert_runtime_check_exact_ids(&input, golden_expected_ids());
}

#[test]
fn validate_script_rejects_unsafe_nested_package_scripts() {
    let mut input = super::helpers::golden();
    replace_validate_with_nested_scripts(&mut input, true);

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-validate-script");
}

#[test]
fn strict_ai_readable_requires_configured_llms_checker_args() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    {
        snapshot.strict_ai_readable = true;
        snapshot.llms_required_sections = vec!["Docs".to_owned()];
        snapshot.llms_required_links = vec!["https://example.com/docs/".to_owned()];
    }
    add_dev_dependency(&mut input, "g3ts-astro-llms");
    add_dev_dependency(&mut input, "g3ts-astro-llms-checks");
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms".to_owned()),
                name: Some("g3ts-astro-llms".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(llms_options()),
                }),
            });
    }
    add_tool_invocation(
        &mut input,
        "g3ts-astro-llms-checks",
        &["--output-dir", "dist"],
        3,
    );

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-checks-validate-script");
}

#[test]
fn broad_agentmarkup_generator_fails() {
    let mut input = super::helpers::golden();
    add_dev_dependency(&mut input, "@agentmarkup/astro");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/broad-crawler-generator-absent");
}

#[test]
fn required_checker_package_in_optional_dependencies_does_not_satisfy_installed_contract() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .dev_dependencies
            .retain(|dependency| dependency != "g3ts-astro-sitemap-checks");
        snapshot
            .optional_dependencies
            .push("g3ts-astro-sitemap-checks".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-checks-package-present");
}

#[test]
fn broad_agentmarkup_generator_fails_from_optional_dependencies() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .optional_dependencies
            .push("@agentmarkup/astro".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/broad-crawler-generator-absent");
}

fn golden_expected_ids() -> &'static [&'static str] {
    &[
        "g3ts-astro-seo/canonical-site-config",
        "g3ts-astro-seo/static-output-config",
        "g3ts-astro-seo/trailing-slash-policy",
        "g3ts-astro-seo/nuasite-checks",
        "g3ts-astro-seo/sitemap-integration-present",
        "g3ts-astro-seo/robots-integration-present",
        "g3ts-astro-seo/sitemap-generator-package-present",
        "g3ts-astro-seo/robots-generator-package-present",
        "g3ts-astro-seo/sitemap-checks-package-present",
        "g3ts-astro-seo/robots-checks-package-present",
        "g3ts-astro-seo/sitemap-checks-validate-script",
        "g3ts-astro-seo/robots-checks-validate-script",
        "g3ts-astro-seo/broad-crawler-generator-absent",
        "g3ts-astro-seo/seo-packages",
        "g3ts-astro-seo/structured-data-check",
        "g3ts-astro-seo/strict-policy-paths",
        "g3ts-astro-seo/policy-helper-surfaces",
        "g3ts-astro-seo/metadata-helper-rule",
        "g3ts-astro-seo/json-ld-helper-rule",
        "g3ts-astro-seo/protected-seo-rule-disables-restricted",
        "g3ts-astro-seo/eslint-disable-inventory",
    ]
}

fn rewrite_validate_output_dir(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    output_dir: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        for invocation in &mut snapshot.script_tool_invocations {
            for pair in invocation.args.chunks_exact_mut(2) {
                if pair[0] == "--output-dir" {
                    pair[1] = output_dir.to_owned();
                }
            }
        }
    }
}

fn replace_validate_with_nested_scripts(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    unsafe_sitemap_child: bool,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .script_commands
            .retain(|command| command.script_name == "build");
        snapshot
            .script_tool_invocations
            .retain(|invocation| invocation.script_name == "build");
        snapshot.script_parse_blockers.clear();

        snapshot.script_tool_invocations.extend([
            package_script_invocation("validate", 0, "build", None, Some(and())),
            package_script_invocation("validate", 1, "check:sitemap", Some(and()), Some(and())),
            package_script_invocation("validate", 2, "check:robots", Some(and()), None),
            astro_build_invocation("build", 0),
            artifact_invocation(
                "check:sitemap",
                "g3ts-astro-sitemap-checks",
                &["--site", "https://example.com", "--output-dir", "dist"],
                if unsafe_sitemap_child {
                    Some(or())
                } else {
                    None
                },
                None,
                0,
            ),
            artifact_invocation(
                "check:robots",
                "g3ts-astro-robots-checks",
                &[
                    "--site",
                    "https://example.com",
                    "--output-dir",
                    "dist",
                    "--sitemap",
                    "https://example.com/sitemap-index.xml",
                ],
                None,
                None,
                0,
            ),
        ]);
    }
}

fn replace_validate_with_single_nested_artifact_script(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .script_commands
            .retain(|command| command.script_name == "build");
        snapshot
            .script_tool_invocations
            .retain(|invocation| invocation.script_name == "build");
        snapshot.script_parse_blockers.clear();

        snapshot.script_tool_invocations.extend([
            package_script_invocation("validate", 0, "check:artifacts", None, None),
            astro_build_invocation("check:artifacts", 0),
            artifact_invocation(
                "check:artifacts",
                "g3ts-astro-sitemap-checks",
                &["--site", "https://example.com", "--output-dir", "dist"],
                Some(and()),
                Some(and()),
                1,
            ),
            artifact_invocation(
                "check:artifacts",
                "g3ts-astro-robots-checks",
                &[
                    "--site",
                    "https://example.com",
                    "--output-dir",
                    "dist",
                    "--sitemap",
                    "https://example.com/sitemap-index.xml",
                ],
                Some(and()),
                None,
                2,
            ),
        ]);
    }
}

fn package_script_invocation(
    script_name: &str,
    command_index: usize,
    target_script: &str,
    preceded_by: Option<g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator>,
    followed_by: Option<g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator>,
) -> g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
    g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        command_index,
        invocation: format!("npm run {target_script}"),
        executable: "package-script".to_owned(),
        args: vec![target_script.to_owned()],
        preceded_by,
        followed_by,
    }
}

fn astro_build_invocation(
    script_name: &str,
    command_index: usize,
) -> g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
    g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        command_index,
        invocation: "astro build".to_owned(),
        executable: "astro".to_owned(),
        args: vec!["build".to_owned()],
        preceded_by: None,
        followed_by: None,
    }
}

fn artifact_invocation(
    script_name: &str,
    executable: &str,
    args: &[&str],
    preceded_by: Option<g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator>,
    followed_by: Option<g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator>,
    command_index: usize,
) -> g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
    g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
        script_name: script_name.to_owned(),
        command_index,
        invocation: format!("{executable} {}", args.join(" ")),
        executable: executable.to_owned(),
        args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        preceded_by,
        followed_by,
    }
}

fn and() -> g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator {
    g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And
}

fn or() -> g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator {
    g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::Or
}

fn llms_options() -> g3ts_astro_seo_types::G3TsAstroStaticValue {
    g3ts_astro_seo_types::G3TsAstroStaticValue::Object(vec![
        g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
            key: "title".to_owned(),
            value: g3ts_astro_seo_types::G3TsAstroStaticValue::String("Example".to_owned()),
        },
        g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
            key: "site".to_owned(),
            value: g3ts_astro_seo_types::G3TsAstroStaticValue::String(
                "https://example.com".to_owned(),
            ),
        },
        g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
            key: "sections".to_owned(),
            value: g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![
                g3ts_astro_seo_types::G3TsAstroStaticValue::Object(vec![
                    g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
                        key: "heading".to_owned(),
                        value: g3ts_astro_seo_types::G3TsAstroStaticValue::String(
                            "Docs".to_owned(),
                        ),
                    },
                    g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
                        key: "links".to_owned(),
                        value: g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![
                            g3ts_astro_seo_types::G3TsAstroStaticValue::Object(vec![
                                g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
                                    key: "title".to_owned(),
                                    value: g3ts_astro_seo_types::G3TsAstroStaticValue::String(
                                        "Docs".to_owned(),
                                    ),
                                },
                                g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
                                    key: "href".to_owned(),
                                    value: g3ts_astro_seo_types::G3TsAstroStaticValue::String(
                                        "/docs/".to_owned(),
                                    ),
                                },
                            ]),
                        ]),
                    },
                ]),
            ]),
        },
    ])
}

fn remove_dev_dependency(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    dependency_name: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .dev_dependencies
            .retain(|dependency| dependency != dependency_name);
    }
}

fn add_dev_dependency(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    dependency_name: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot.dev_dependencies.push(dependency_name.to_owned());
    }
}

fn add_tool_invocation(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    executable: &str,
    args: &[&str],
    command_index: usize,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot.script_tool_invocations.push(
            g3ts_astro_seo_types::G3TsAstroPackageScriptToolInvocation {
                script_name: "validate".to_owned(),
                command_index,
                invocation: format!("{executable} {}", args.join(" ")),
                executable: executable.to_owned(),
                args: args.iter().map(|arg| (*arg).to_owned()).collect(),
                preceded_by: Some(
                    g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                ),
                followed_by: None,
            },
        );
    }
}

fn remove_tool_invocation(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    executable: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    {
        snapshot
            .script_tool_invocations
            .retain(|invocation| invocation.executable != executable);
    }
}

fn remove_integration(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    source_module: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    {
        snapshot
            .integrations
            .retain(|integration| integration.source_module.as_deref() != Some(source_module));
    }
}
