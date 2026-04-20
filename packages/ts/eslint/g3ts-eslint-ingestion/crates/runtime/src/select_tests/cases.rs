use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_ingestion_assertions::select as assertions;

#[test]
fn selects_root_config_by_official_precedence() {
    let root = super::helpers::fake_root();
    let crawl =
        super::helpers::crawl_with_entries(&root, &["eslint.config.ts", "eslint.config.js"]);

    let selected = super::super::select_active_root_eslint_config(&crawl)
        .expect("a root eslint config should be selected");

    assertions::assert_selected_rel_path(selected, "eslint.config.js");
}

#[test]
fn ts_source_probe_prefers_real_source_over_scripts_ts() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "eslint.config.mjs",
            "scripts/extract-content-schema.ts",
            "src/app/page.tsx",
        ],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");

    assert_eq!(
        ts_source.rel_path, "src/app/page.tsx",
        "TS source probe should prefer real source files over scripts/*.ts"
    );
}
