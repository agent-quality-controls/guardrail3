use g3ts_astro_media_config_checks_assertions::run as assertions;

#[test]
fn golden_reports_media_inventory_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-media/strict-policy-configured",
            "g3ts-astro-media/policy-paths-valid",
            "g3ts-astro-media/media-assets-package-present",
            "g3ts-astro-media/media-policy-plugin-package-present",
            "g3ts-astro-media/eslint-comments-plugin-package-present",
            "g3ts-astro-media/media-assets-integration-wired",
            "g3ts-astro-media/media-build-validation-runs",
            "g3ts-astro-media/media-policy-plugin-wired",
            "g3ts-astro-media/no-raw-public-image-paths-rule",
            "g3ts-astro-media/no-inline-image-alt-rule",
            "g3ts-astro-media/require-content-image-key-rule",
            "g3ts-astro-media/require-approved-media-helper-rule",
            "g3ts-astro-media/protected-media-rule-disables-restricted",
        ],
    );
}

#[test]
fn missing_media_policy_fails() {
    let mut input = super::helpers::golden();
    input.integration_contracts[0].astro_policy =
        g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::MissingMediaPolicy {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
        };

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_assets_integration_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.integrations.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/media-assets-integration-wired",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn invalid_output_asset_path_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.favicon = "favicon.svg".to_owned();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/policy-paths-valid",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn invalid_app_relative_source_glob_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.public_source_globs = vec!["/src/**/*.astro".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/policy-paths-valid",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn external_media_helper_module_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.media_helper_modules = vec!["https://example.com/media.ts".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/policy-paths-valid",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn whitespace_only_policy_arrays_fail() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.approved_media_helpers = vec![" ".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_allow_svg_icons_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.allow_svg_icons = None;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_allowed_public_image_paths_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.allowed_public_image_paths = Vec::new();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn unknown_media_policy_fields_fail() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.extra_fields = vec!["allow_raw_public_images".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_policy_rule_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroMediaEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot
        .public_media_policy_rules
        .retain(|rule| rule != "astro-media-policy/no-inline-image-alt");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/no-inline-image-alt-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_delegated_package_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_media_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot
        .dev_dependencies
        .retain(|package| package != "g3ts-eslint-plugin-astro-media-policy");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-media/media-policy-plugin-package-present",
        guardrail3_check_types::G3Severity::Error,
    );
}
