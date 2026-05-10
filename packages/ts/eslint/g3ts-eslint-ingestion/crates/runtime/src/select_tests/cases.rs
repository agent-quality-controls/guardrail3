use eslint_config_parser::types::EslintProbeKind;
use g3_workspace_crawl::G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState;
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

#[test]
fn ts_source_probe_skips_nested_scripts_directories() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "eslint.config.mjs",
            "apps/landing/scripts/extract-content-schema.ts",
            "apps/landing/src/lib/env.ts",
        ],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");

    assert_eq!(
        ts_source.rel_path, "apps/landing/src/lib/env.ts",
        "TS source probe should skip nested scripts directories as fallback candidates"
    );
}

#[test]
fn tsx_probe_is_added_only_for_real_tsx_source_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &["eslint.config.mjs", "src/index.ts", "src/app/page.tsx"],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let tsx_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsxSource)
        .expect("TSX source probe should exist when a real .tsx file exists");

    assert_eq!(
        tsx_source.rel_path, "src/app/page.tsx",
        "TSX source probe path mismatch"
    );
}

#[test]
fn probe_selection_skips_ignored_and_unreadable_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_custom_entries(
        &root,
        &[
            ("eslint.config.mjs", G3WorkspaceIgnoreState::Included, true),
            ("src/generated.ts", G3WorkspaceIgnoreState::Ignored, true),
            ("src/unreadable.ts", G3WorkspaceIgnoreState::Included, false),
            ("src/index.ts", G3WorkspaceIgnoreState::Included, true),
            ("src/app/page.tsx", G3WorkspaceIgnoreState::Included, true),
            ("src/index.js", G3WorkspaceIgnoreState::Included, true),
        ],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");
    let js_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::JsSource)
        .expect("JS source probe should exist");

    assert_eq!(ts_source.rel_path, "src/index.ts");
    assert_eq!(js_source.rel_path, "src/index.js");
}

#[test]
fn js_source_probe_skips_test_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "eslint.config.mjs",
            "src/index.test.js",
            "src/index.js",
            "src/index.ts",
        ],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let js_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::JsSource)
        .expect("JS source probe should exist");

    assert_eq!(js_source.rel_path, "src/index.js");
}

#[test]
fn fallback_probes_skip_config_modules() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "eslint.config.mjs",
            "vite.config.ts",
            "next.config.js",
            "src/index.ts",
            "src/index.js",
        ],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");
    let js_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::JsSource)
        .expect("JS source probe should exist");

    assert_eq!(ts_source.rel_path, "src/index.ts");
    assert_eq!(js_source.rel_path, "src/index.js");
}

#[test]
fn probe_selection_is_scoped_to_selected_config_root() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "packages/alpha/eslint.config.mjs",
            "packages/alpha/src/index.ts",
            "packages/alpha/src/index.js",
            "packages/beta/src/index.ts",
            "packages/beta/src/index.js",
        ],
    );

    let probes = super::super::probe_targets(&crawl, "packages/alpha/eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");
    let js_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::JsSource)
        .expect("JS source probe should exist");

    assert_eq!(ts_source.rel_path, "packages/alpha/src/index.ts");
    assert_eq!(js_source.rel_path, "packages/alpha/src/index.js");
}
