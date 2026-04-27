use g3ts_astro_content_config_checks_assertions::run as assertions;

#[test]
fn golden_content_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-content/pipeline-plugin-package-present",
            "g3ts-astro-content/content-adapter-rule",
            "g3ts-astro-content/inline-copy-rule",
            "g3ts-astro-content/strict-content-policy",
            "g3ts-astro-content/strict-policy-paths",
            "g3ts-astro-content/route-scope-overlap",
            "g3ts-astro-content/policy-eslint-coverage",
            "g3ts-astro-content/content-adapter-exists",
            "g3ts-astro-content/content-adapter-astro-content",
        ],
    );
}

#[test]
fn content_adapter_rule_accepts_equivalent_recursive_file_globs() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    let glob = "src/lib/content/**/*".to_owned();
    snapshot.astro_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.ts_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.tsx_source_effective_content_adapter_modules = vec![glob];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn content_adapter_rule_rejects_unrelated_recursive_file_globs() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    let glob = "src/other-content/**/*".to_owned();
    snapshot.astro_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.ts_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.tsx_source_effective_content_adapter_modules = vec![glob];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn content_adapter_rule_rejects_missing_tsx_lane_coverage() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    snapshot
        .tsx_source_effective_content_adapter_modules
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}
