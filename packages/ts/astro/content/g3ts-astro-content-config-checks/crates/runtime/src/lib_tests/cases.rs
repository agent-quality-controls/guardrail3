use g3ts_astro_content_config_checks_assertions::run as assertions;

#[test]
fn golden_content_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "TS-ASTRO-CONTENT-CONFIG-17",
            "TS-ASTRO-CONTENT-CONFIG-18",
            "TS-ASTRO-CONTENT-CONFIG-19",
            "TS-ASTRO-CONTENT-CONFIG-23",
            "TS-ASTRO-CONTENT-CONFIG-24",
            "TS-ASTRO-CONTENT-CONFIG-25",
            "TS-ASTRO-CONTENT-CONFIG-26",
            "TS-ASTRO-CONTENT-CONFIG-27",
            "TS-ASTRO-CONTENT-CONFIG-28",
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
        "TS-ASTRO-CONTENT-CONFIG-18",
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
        "TS-ASTRO-CONTENT-CONFIG-18",
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
        "TS-ASTRO-CONTENT-CONFIG-18",
        guardrail3_check_types::G3Severity::Error,
    );
}
