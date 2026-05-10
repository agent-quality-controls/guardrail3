use std::collections::BTreeMap;

use g3ts_astro_media_types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroIntegrationSnapshot, G3TsAstroMediaConfigChecksInput,
    G3TsAstroMediaEslintPluginContractInput, G3TsAstroMediaEslintSurfaceSnapshot,
    G3TsAstroMediaEslintSurfaceState, G3TsAstroMediaIntegrationContractInput,
    G3TsAstroMediaPolicySnapshot, G3TsAstroMediaPolicySurfaceState,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroStaticObjectProperty, G3TsAstroStaticValue,
};

/// Returns a mutable reference to the parsed media policy snapshot of the first integration contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first integration contract astro_policy must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn policy_snapshot_mut(
    input: &mut G3TsAstroMediaConfigChecksInput,
) -> &mut G3TsAstroMediaPolicySnapshot {
    let policy = &mut input.integration_contracts[0].astro_policy;
    let G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } = policy else {
        panic!("golden media policy should be parsed");
    };
    snapshot
}

/// Returns a mutable reference to the parsed astro config snapshot of the first integration contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first integration contract astro_config must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn astro_config_snapshot_mut(
    input: &mut G3TsAstroMediaConfigChecksInput,
) -> &mut G3TsAstroConfigSurfaceSnapshot {
    let config = &mut input.integration_contracts[0].astro_config;
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } = config else {
        panic!("golden astro config should be parsed");
    };
    snapshot
}

/// Returns a mutable reference to the parsed eslint snapshot of the first eslint contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first eslint contract config must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn eslint_snapshot_mut(
    input: &mut G3TsAstroMediaConfigChecksInput,
) -> &mut G3TsAstroMediaEslintSurfaceSnapshot {
    let config = &mut input.eslint_contracts[0].config;
    let G3TsAstroMediaEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden media eslint config should be parsed");
    };
    snapshot
}

/// Returns a mutable reference to the parsed package snapshot of the first integration contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first integration contract package must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn package_snapshot_mut(
    input: &mut G3TsAstroMediaConfigChecksInput,
) -> &mut G3TsAstroPackageSurfaceSnapshot {
    let package = &mut input.integration_contracts[0].package;
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } = package else {
        panic!("golden media package should be parsed");
    };
    snapshot
}

/// Sets the `astro_policy` of the first integration contract.
///
/// Test-only helper.
#[expect(
    clippy::indexing_slicing,
    reason = "golden fixture invariant: first integration contract must exist; mismatch is a test setup failure"
)]
pub(super) fn set_first_integration_policy(
    input: &mut G3TsAstroMediaConfigChecksInput,
    policy: G3TsAstroMediaPolicySurfaceState,
) {
    input.integration_contracts[0].astro_policy = policy;
}

pub(super) fn golden() -> G3TsAstroMediaConfigChecksInput {
    G3TsAstroMediaConfigChecksInput {
        integration_contracts: vec![G3TsAstroMediaIntegrationContractInput {
            app_root_rel_path: "apps/landing".to_owned(),
            package: package(),
            astro_config: astro_config(),
            astro_policy: policy(),
        }],
        eslint_contracts: vec![G3TsAstroMediaEslintPluginContractInput {
            app_root_rel_path: "apps/landing".to_owned(),
            config: eslint_config(),
            astro_policy: policy(),
        }],
    }
}

fn package() -> G3TsAstroPackageSurfaceState {
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "apps/landing/package.json".to_owned(),
            package_name: Some("landing".to_owned()),
            dependencies: vec!["g3ts-astro-media-assets".to_owned()],
            dev_dependencies: vec![
                "g3ts-eslint-plugin-astro-media-policy".to_owned(),
                "@eslint-community/eslint-plugin-eslint-comments".to_owned(),
            ],
            optional_dependencies: Vec::new(),
            peer_dependencies: Vec::new(),
            script_names: vec!["validate".to_owned()],
            script_bodies: vec![("validate".to_owned(), "astro build".to_owned())],
            script_commands: Vec::new(),
            script_tool_invocations: vec![G3TsAstroPackageScriptToolInvocation {
                script_name: "validate".to_owned(),
                command_index: 0,
                invocation: "astro build".to_owned(),
                executable: "astro".to_owned(),
                args: vec!["build".to_owned()],
                preceded_by: None,
                followed_by: None,
            }],
            script_parse_blockers: Vec::new(),
        },
    }
}

fn astro_config() -> G3TsAstroConfigSurfaceState {
    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: "apps/landing/astro.config.mjs".to_owned(),
            integrations: vec![G3TsAstroIntegrationSnapshot {
                source_module: Some("g3ts-astro-media-assets".to_owned()),
                name: None,
                imported_name: Some("default".to_owned()),
                call: Some(G3TsAstroCallSnapshot {
                    first_arg: Some(G3TsAstroStaticValue::Object(vec![
                        prop(
                            "favicon",
                            G3TsAstroStaticValue::String("/favicon.svg".to_owned()),
                        ),
                        prop(
                            "appIcons",
                            G3TsAstroStaticValue::Array(vec![G3TsAstroStaticValue::String(
                                "/apple-touch-icon.png".to_owned(),
                            )]),
                        ),
                        prop(
                            "defaultSocialImage",
                            G3TsAstroStaticValue::String("/og/default.png".to_owned()),
                        ),
                        prop("allowSvgIcons", G3TsAstroStaticValue::Bool(true)),
                    ])),
                }),
            }],
        },
    }
}

fn prop(key: &str, value: G3TsAstroStaticValue) -> G3TsAstroStaticObjectProperty {
    G3TsAstroStaticObjectProperty {
        key: key.to_owned(),
        value,
    }
}

fn policy() -> G3TsAstroMediaPolicySurfaceState {
    G3TsAstroMediaPolicySurfaceState::Parsed {
        snapshot: G3TsAstroMediaPolicySnapshot {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
            favicon: "/favicon.svg".to_owned(),
            app_icons: vec!["/apple-touch-icon.png".to_owned()],
            default_social_image: "/og/default.png".to_owned(),
            allow_svg_icons: Some(true),
            public_source_globs: vec!["src/**/*.{astro,ts,tsx}".to_owned()],
            media_helper_modules: vec!["src/media/images.ts".to_owned()],
            approved_media_helpers: vec!["imageMetadata".to_owned()],
            content_image_components: vec!["ArticleImage".to_owned()],
            content_image_key_props: vec!["image".to_owned()],
            banned_image_source_props: vec!["src".to_owned()],
            banned_image_alt_props: vec!["alt".to_owned()],
            allowed_public_image_paths: vec!["/favicon.svg".to_owned()],
            checked_image_extensions: vec![".jpg".to_owned(), ".png".to_owned(), ".svg".to_owned()],
            metadata_image_property_names: vec!["image".to_owned(), "ogImage".to_owned()],
            extra_fields: Vec::new(),
        },
    }
}

fn eslint_config() -> G3TsAstroMediaEslintSurfaceState {
    let mut packages = BTreeMap::new();
    let _ = packages.insert(
        "astro-media-policy".to_owned(),
        vec!["g3ts-eslint-plugin-astro-media-policy".to_owned()],
    );

    G3TsAstroMediaEslintSurfaceState::Parsed {
        snapshot: G3TsAstroMediaEslintSurfaceSnapshot {
            rel_path: "apps/landing/eslint.config.mjs".to_owned(),
            public_probe_present: true,
            public_probe_ignored: false,
            public_plugins: vec!["astro-media-policy".to_owned()],
            public_plugin_package_names: packages,
            public_error_rules: vec![
                "astro-media-policy/no-raw-public-image-paths".to_owned(),
                "astro-media-policy/no-inline-image-alt".to_owned(),
                "astro-media-policy/require-content-image-key".to_owned(),
                "astro-media-policy/require-approved-media-helper".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            public_restricted_disable_patterns: vec!["astro-media-policy/*".to_owned()],
            public_media_policy_rules: vec![
                "astro-media-policy/no-raw-public-image-paths".to_owned(),
                "astro-media-policy/no-inline-image-alt".to_owned(),
                "astro-media-policy/require-content-image-key".to_owned(),
                "astro-media-policy/require-approved-media-helper".to_owned(),
            ],
        },
    }
}
