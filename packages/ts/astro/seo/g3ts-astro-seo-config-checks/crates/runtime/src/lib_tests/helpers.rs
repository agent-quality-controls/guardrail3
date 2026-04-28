use g3ts_astro_seo_types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroIntegrationSnapshot, G3TsAstroOutputMode, G3TsAstroPackageScriptCommand,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroSeoApprovedSourcePaths, G3TsAstroSeoConfigChecksInput,
    G3TsAstroSeoEslintPluginContractInput, G3TsAstroSeoEslintSurfaceSnapshot,
    G3TsAstroSeoEslintSurfaceState, G3TsAstroSeoIntegrationContractInput,
    G3TsAstroSeoPolicySnapshot, G3TsAstroSeoPolicySurfaceState, G3TsAstroStaticObjectProperty,
    G3TsAstroStaticValue, G3TsAstroTrailingSlashPolicy,
};

pub(super) fn golden() -> G3TsAstroSeoConfigChecksInput {
    G3TsAstroSeoConfigChecksInput {
        integration_contracts: vec![G3TsAstroSeoIntegrationContractInput {
            app_root_rel_path: ".".to_owned(),
            seo_sources: G3TsAstroSeoApprovedSourcePaths {
                metadata_helpers: vec!["src/seo/metadata.ts".to_owned()],
                missing_metadata_helpers: Vec::new(),
                json_ld_helpers: vec!["src/seo/json-ld.ts".to_owned()],
                missing_json_ld_helpers: Vec::new(),
            },
            package: package(),
            astro_config: astro_config(),
            astro_policy: astro_policy(),
        }],
        eslint_contracts: vec![G3TsAstroSeoEslintPluginContractInput {
            app_root_rel_path: ".".to_owned(),
            config: eslint_config(),
        }],
        missing_metadata_helper_sources: Vec::new(),
        missing_json_ld_helper_sources: Vec::new(),
        eslint_directives: Vec::new(),
    }
}

fn package() -> G3TsAstroPackageSurfaceState {
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            package_name: Some("landing".to_owned()),
            dependencies: Vec::new(),
            dev_dependencies: vec![
                "@astrojs/sitemap".to_owned(),
                "astro-robots".to_owned(),
                "g3ts-astro-sitemap-checks".to_owned(),
                "g3ts-astro-robots-checks".to_owned(),
                "@nuasite/checks".to_owned(),
                "g3ts-astro-nuasite-checks".to_owned(),
                "schema-dts".to_owned(),
            ],
            optional_dependencies: Vec::new(),
            peer_dependencies: Vec::new(),
            script_names: vec!["build".to_owned(), "validate".to_owned()],
            script_bodies: vec![
                ("build".to_owned(), "astro build".to_owned()),
                (
                    "validate".to_owned(),
                    "astro build && g3ts-astro-sitemap-checks --site https://example.com --output-dir dist && g3ts-astro-robots-checks --site https://example.com --output-dir dist --sitemap https://example.com/sitemap-index.xml"
                        .to_owned(),
                ),
            ],
            script_commands: vec![
                G3TsAstroPackageScriptCommand {
                    script_name: "build".to_owned(),
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "g3ts-astro-sitemap-checks --site https://example.com --output-dir dist".to_owned(),
                    executable: "g3ts-astro-sitemap-checks".to_owned(),
                    args: vec![
                        "--site".to_owned(),
                        "https://example.com".to_owned(),
                        "--output-dir".to_owned(),
                        "dist".to_owned(),
                    ],
                    preceded_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "g3ts-astro-robots-checks --site https://example.com --output-dir dist --sitemap https://example.com/sitemap-index.xml".to_owned(),
                    executable: "g3ts-astro-robots-checks".to_owned(),
                    args: vec![
                        "--site".to_owned(),
                        "https://example.com".to_owned(),
                        "--output-dir".to_owned(),
                        "dist".to_owned(),
                        "--sitemap".to_owned(),
                        "https://example.com/sitemap-index.xml".to_owned(),
                    ],
                    preceded_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
            ],
            script_tool_invocations: vec![
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "build".to_owned(),
                    command_index: 0,
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 0,
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                    followed_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 1,
                    invocation: "g3ts-astro-sitemap-checks --site https://example.com --output-dir dist".to_owned(),
                    executable: "g3ts-astro-sitemap-checks".to_owned(),
                    args: vec![
                        "--site".to_owned(),
                        "https://example.com".to_owned(),
                        "--output-dir".to_owned(),
                        "dist".to_owned(),
                    ],
                    preceded_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 2,
                    invocation: "g3ts-astro-robots-checks --site https://example.com --output-dir dist --sitemap https://example.com/sitemap-index.xml".to_owned(),
                    executable: "g3ts-astro-robots-checks".to_owned(),
                    args: vec![
                        "--site".to_owned(),
                        "https://example.com".to_owned(),
                        "--output-dir".to_owned(),
                        "dist".to_owned(),
                        "--sitemap".to_owned(),
                        "https://example.com/sitemap-index.xml".to_owned(),
                    ],
                    preceded_by: Some(
                        g3ts_astro_seo_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: None,
                },
            ],
            script_parse_blockers: Vec::new(),
        },
    }
}

fn astro_config() -> G3TsAstroConfigSurfaceState {
    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: "astro.config.mjs".to_owned(),
            site: Some("https://example.com".to_owned()),
            output: Some(G3TsAstroOutputMode::Static),
            out_dir: None,
            trailing_slash: Some(G3TsAstroTrailingSlashPolicy::Always),
            integrations: vec![
                integration("@astrojs/sitemap", None),
                integration("astro-robots", None),
                integration("@nuasite/checks", Some(nuasite_options())),
            ],
            adapter: None,
        },
    }
}

fn astro_policy() -> G3TsAstroSeoPolicySurfaceState {
    G3TsAstroSeoPolicySurfaceState::Parsed {
        snapshot: G3TsAstroSeoPolicySnapshot {
            rel_path: "guardrail3-ts.toml".to_owned(),
            metadata_helpers: vec!["src/seo/metadata".to_owned()],
            json_ld_helpers: vec!["src/seo/json-ld".to_owned()],
            strict_ai_readable: false,
            llms_required_sections: Vec::new(),
            llms_required_links: Vec::new(),
        },
    }
}

fn eslint_config() -> G3TsAstroSeoEslintSurfaceState {
    G3TsAstroSeoEslintSurfaceState::Parsed {
        snapshot: G3TsAstroSeoEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            astro_source_probe_present: true,
            ts_source_probe_present: true,
            tsx_source_probe_present: true,
            astro_source_effective_metadata_helper_rules: vec![
                "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
            ],
            ts_source_effective_metadata_helper_rules: vec![
                "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
            ],
            tsx_source_effective_metadata_helper_rules: vec![
                "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
            ],
            astro_source_effective_json_ld_helper_rules: vec![
                "astro-pipeline/require-approved-json-ld-helper-in-routes".to_owned(),
            ],
            ts_source_effective_json_ld_helper_rules: vec![
                "astro-pipeline/require-approved-json-ld-helper-in-routes".to_owned(),
            ],
            tsx_source_effective_json_ld_helper_rules: vec![
                "astro-pipeline/require-approved-json-ld-helper-in-routes".to_owned(),
            ],
            astro_source_warn_or_error_rules: seo_warn_or_error_rules(),
            ts_source_warn_or_error_rules: seo_warn_or_error_rules(),
            tsx_source_warn_or_error_rules: seo_warn_or_error_rules(),
            astro_source_restricted_disable_patterns: seo_restricted_disable_patterns(),
            ts_source_restricted_disable_patterns: seo_restricted_disable_patterns(),
            tsx_source_restricted_disable_patterns: seo_restricted_disable_patterns(),
        },
    }
}

fn seo_warn_or_error_rules() -> Vec<String> {
    vec![
        "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
        "astro-pipeline/require-approved-json-ld-helper-in-routes".to_owned(),
        "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
    ]
}

fn seo_restricted_disable_patterns() -> Vec<String> {
    vec![
        "astro-pipeline/require-approved-metadata-helper-in-routes".to_owned(),
        "astro-pipeline/require-approved-json-ld-helper-in-routes".to_owned(),
    ]
}

fn integration(
    source_module: &str,
    first_arg: Option<G3TsAstroStaticValue>,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: Some(source_module.to_owned()),
        name: Some(source_module.to_owned()),
        imported_name: None,
        call: Some(G3TsAstroCallSnapshot { first_arg }),
    }
}

fn nuasite_options() -> G3TsAstroStaticValue {
    G3TsAstroStaticValue::Object(vec![
        property("mode", G3TsAstroStaticValue::String("full".to_owned())),
        property("failOnError", G3TsAstroStaticValue::Bool(true)),
        property("failOnWarning", G3TsAstroStaticValue::Bool(true)),
        property("reportJson", G3TsAstroStaticValue::Bool(true)),
        property("ai", G3TsAstroStaticValue::Bool(false)),
        property(
            "customChecks",
            G3TsAstroStaticValue::Array(vec![G3TsAstroStaticValue::ImportedIdentifier {
                local_name: "structuredDataPresentCheck".to_owned(),
                source_module: Some("g3ts-astro-nuasite-checks".to_owned()),
                imported_name: Some("structuredDataPresentCheck".to_owned()),
            }]),
        ),
    ])
}

fn property(key: &str, value: G3TsAstroStaticValue) -> G3TsAstroStaticObjectProperty {
    G3TsAstroStaticObjectProperty {
        key: key.to_owned(),
        value,
    }
}
