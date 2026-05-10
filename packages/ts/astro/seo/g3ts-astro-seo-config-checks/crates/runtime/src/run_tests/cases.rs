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
            "g3ts-astro-seo/sitemap-auditor-package-present",
            "g3ts-astro-seo/robots-auditor-package-present",
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
    let config = &mut input
        .eslint_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one eslint_contracts entry")
        .config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        unreachable!("golden seo eslint config should be parsed");
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
    let config = &mut input
        .eslint_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one eslint_contracts entry")
        .config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        unreachable!("golden seo eslint config should be parsed");
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
    let config = &mut input
        .eslint_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one eslint_contracts entry")
        .config;
    let g3ts_astro_seo_types::G3TsAstroSeoEslintSurfaceState::Parsed { snapshot } = config else {
        unreachable!("golden seo eslint config should be parsed");
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
        .push(g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput::new(
            "src/pages/index.astro".to_owned(),
            "DisableNextLine".to_owned(),
            vec!["astro-pipeline/require-approved-metadata-helper-in-routes".to_owned()],
            false,
            20,
            Some(21),
            None,
        ));

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
        .push(g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput::new(
            "src/pages/index.astro".to_owned(),
            "ParseError".to_owned(),
            Vec::new(),
            false,
            0,
            None,
            Some("ambiguous directive syntax".to_owned()),
        ));

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-seo/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_site_fails_canonical_site_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.site = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/canonical-site-config");
}

#[test]
fn http_site_fails_canonical_site_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.site = Some("http://example.com".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/canonical-site-config");
}

#[test]
fn missing_static_output_fails_static_output_config() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.output = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/static-output-config");
}

#[test]
fn missing_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.trailing_slash = None;
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn ignored_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.trailing_slash = Some(g3ts_astro_seo_types::G3TsAstroTrailingSlashPolicy::Ignore);
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn missing_sitemap_auditor_package_fails() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "g3ts-astro-sitemap-auditor");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-auditor-package-present");
}

#[test]
fn missing_sitemap_generator_package_fails_integration_rule() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "@astrojs/sitemap");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-generator-package-present");
}

#[test]
fn missing_sitemap_integration_fails() {
    let mut input = super::helpers::golden();
    remove_integration(&mut input, "@astrojs/sitemap");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-integration-present");
}

#[test]
fn missing_robots_auditor_package_fails() {
    let mut input = super::helpers::golden();
    remove_dev_dependency(&mut input, "g3ts-astro-robots-auditor");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-auditor-package-present");
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
fn never_trailing_slash_fails_trailing_slash_policy() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot.trailing_slash = Some(g3ts_astro_seo_types::G3TsAstroTrailingSlashPolicy::Never);
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/trailing-slash-policy");
}

#[test]
fn strict_ai_readable_requires_llms_generator_auditor_and_integrations() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_policy
    {
        snapshot.strict_ai_readable = true;
        snapshot.llms_required_sections = vec!["Docs".to_owned()];
        snapshot.llms_required_links = vec!["https://example.com/docs/".to_owned()];
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-integration-present");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-generator-package-present");
    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-auditor-package-present");
}

#[test]
fn strict_ai_readable_passes_when_llms_generator_auditor_and_integrations_are_present() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_policy
    {
        snapshot.strict_ai_readable = true;
    }
    add_dev_dependency(&mut input, "g3ts-astro-llms-generator");
    add_dev_dependency(&mut input, "g3ts-astro-llms-auditor");
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms-generator".to_owned()),
                name: Some("g3ts-astro-llms-generator".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(llms_generator_options()),
                }),
            });
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms-auditor".to_owned()),
                name: Some("g3ts-astro-llms-auditor".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(llms_auditor_options()),
                }),
            });
    }

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
            "g3ts-astro-seo/sitemap-auditor-package-present",
            "g3ts-astro-seo/robots-auditor-package-present",
            "g3ts-astro-seo/llms-generator-package-present",
            "g3ts-astro-seo/llms-auditor-package-present",
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
fn sitemap_auditor_config_rejects_unknown_keys() {
    let mut input = super::helpers::golden();
    append_integration_option(
        &mut input,
        "g3ts-astro-sitemap-auditor",
        static_property(
            "extra",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Bool(true),
        ),
    );

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-integration-present");
}

#[test]
fn robots_auditor_config_rejects_unknown_keys() {
    let mut input = super::helpers::golden();
    append_integration_option(
        &mut input,
        "g3ts-astro-robots-auditor",
        static_property(
            "approvedSitemapUrls",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
    );

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/robots-integration-present");
}

#[test]
fn llms_configs_reject_unknown_keys() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_policy
    {
        snapshot.strict_ai_readable = true;
    }
    add_dev_dependency(&mut input, "g3ts-astro-llms-generator");
    add_dev_dependency(&mut input, "g3ts-astro-llms-auditor");
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms-generator".to_owned()),
                name: Some("g3ts-astro-llms-generator".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(g3ts_astro_seo_types::G3TsAstroStaticValue::Object({
                        let g3ts_astro_seo_types::G3TsAstroStaticValue::Object(mut properties) =
                            llms_generator_options()
                        else {
                            unreachable!(
                                "llms_generator_options must return an object literal for tests"
                            );
                        };
                        properties.push(static_property(
                            "extra",
                            g3ts_astro_seo_types::G3TsAstroStaticValue::Bool(true),
                        ));
                        properties
                    })),
                }),
            });
        snapshot
            .integrations
            .push(g3ts_astro_seo_types::G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-llms-auditor".to_owned()),
                name: Some("g3ts-astro-llms-auditor".to_owned()),
                imported_name: None,
                call: Some(g3ts_astro_seo_types::G3TsAstroCallSnapshot {
                    first_arg: Some(llms_auditor_options()),
                }),
            });
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/llms-integration-present");
}

#[test]
fn broad_agentmarkup_generator_fails() {
    let mut input = super::helpers::golden();
    add_dev_dependency(&mut input, "@agentmarkup/astro");

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/broad-crawler-generator-absent");
}

#[test]
fn required_auditor_package_in_optional_dependencies_does_not_satisfy_installed_contract() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .package
    {
        snapshot
            .dev_dependencies
            .retain(|dependency| dependency != "g3ts-astro-sitemap-auditor");
        snapshot
            .optional_dependencies
            .push("g3ts-astro-sitemap-auditor".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/sitemap-auditor-package-present");
}

#[test]
fn broad_agentmarkup_generator_fails_from_optional_dependencies() {
    let mut input = super::helpers::golden();
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .package
    {
        snapshot
            .optional_dependencies
            .push("@agentmarkup/astro".to_owned());
    }

    assertions::assert_runtime_error_id(&input, "g3ts-astro-seo/broad-crawler-generator-absent");
}

fn llms_generator_options() -> g3ts_astro_seo_types::G3TsAstroStaticValue {
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

fn llms_auditor_options() -> g3ts_astro_seo_types::G3TsAstroStaticValue {
    g3ts_astro_seo_types::G3TsAstroStaticValue::Object(vec![
        static_property(
            "site",
            g3ts_astro_seo_types::G3TsAstroStaticValue::String("https://example.com".to_owned()),
        ),
        static_property(
            "requiredSections",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
        static_property(
            "requiredRoutePatterns",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
        static_property(
            "allowedExternalUrls",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
        static_property(
            "allowedNonPageUrls",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
        static_property(
            "ignoredHtmlFiles",
            g3ts_astro_seo_types::G3TsAstroStaticValue::Array(vec![]),
        ),
    ])
}

fn static_property(
    key: &str,
    value: g3ts_astro_seo_types::G3TsAstroStaticValue,
) -> g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
    g3ts_astro_seo_types::G3TsAstroStaticObjectProperty {
        key: key.to_owned(),
        value,
    }
}

fn append_integration_option(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    module: &str,
    property: g3ts_astro_seo_types::G3TsAstroStaticObjectProperty,
) {
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        let Some(integration) = snapshot
            .integrations
            .iter_mut()
            .find(|integration| integration.source_module.as_deref() == Some(module))
        else {
            unreachable!("golden config should contain integration {module}");
        };
        let Some(call) = &mut integration.call else {
            unreachable!("golden integration should be called");
        };
        let Some(g3ts_astro_seo_types::G3TsAstroStaticValue::Object(properties)) =
            &mut call.first_arg
        else {
            unreachable!("golden integration should have object options");
        };
        properties.push(property);
    }
}

fn remove_dev_dependency(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    dependency_name: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .package
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
    if let g3ts_astro_seo_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .package
    {
        snapshot.dev_dependencies.push(dependency_name.to_owned());
    }
}

fn remove_integration(
    input: &mut g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    source_module: &str,
) {
    if let g3ts_astro_seo_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } = &mut input
        .integration_contracts
        .first_mut()
        .expect("seo test fixture must declare at least one integration_contracts entry")
        .astro_config
    {
        snapshot
            .integrations
            .retain(|integration| integration.source_module.as_deref() != Some(source_module));
    }
}
