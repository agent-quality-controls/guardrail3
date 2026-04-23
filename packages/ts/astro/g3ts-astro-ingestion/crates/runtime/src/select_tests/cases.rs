use eslint_config_parser::types::EslintProbeKind;
use g3_workspace_crawl::G3WorkspaceIgnoreState;

#[test]
fn selects_root_astro_config_by_official_precedence() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &["astro.config.ts", "astro.config.js", "src/pages/index.ts"],
    );

    let selected = super::super::select_root_astro_config(&crawl)
        .expect("a root astro config should be selected");

    assert_eq!(selected.path.rel_path, "astro.config.js");
}

#[test]
fn content_mode_selection_detects_content_and_live_configs() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "src/content.config.ts",
            "src/live.config.ts",
            "src/content/posts/post.mdx",
        ],
    );

    assert_eq!(
        super::super::select_content_config(&crawl)
            .expect("content config should be selected")
            .path
            .rel_path,
        "src/content.config.ts"
    );
    assert_eq!(
        super::super::select_live_config(&crawl)
            .expect("live config should be selected")
            .path
            .rel_path,
        "src/live.config.ts"
    );
    assert!(super::super::has_content_files(&crawl));
}

#[test]
fn route_markdown_pages_only_include_src_pages_markdown_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "src/pages/index.ts",
            "src/pages/about.mdx",
            "src/pages/legal.md",
            "src/content/posts/post.mdx",
        ],
    );

    assert_eq!(
        super::super::route_markdown_pages(&crawl),
        vec!["src/pages/about.mdx".to_owned(), "src/pages/legal.md".to_owned()]
    );
}

#[test]
fn source_like_entries_skip_ignored_and_unreadable_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_custom_entries(
        &root,
        &[
            ("src/lib/ok.ts", G3WorkspaceIgnoreState::Included, true),
            ("src/lib/unreadable.ts", G3WorkspaceIgnoreState::Included, false),
            ("src/lib/ignored.ts", G3WorkspaceIgnoreState::Ignored, true),
            ("src/content/posts/post.mdx", G3WorkspaceIgnoreState::Included, true),
        ],
    );

    let selected = super::super::source_like_entries(&crawl)
        .into_iter()
        .map(|entry| entry.path.rel_path.clone())
        .collect::<Vec<_>>();

    assert_eq!(selected, vec!["src/lib/ok.ts".to_owned()]);
}

#[test]
fn eslint_probe_targets_use_real_source_files() {
    let root = super::helpers::fake_root();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &["eslint.config.mjs", "src/index.ts", "src/app/page.tsx"],
    );

    let probes = super::super::probe_targets(&crawl, "eslint.config.mjs");
    let ts_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("TS source probe should exist");
    let tsx_source = probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsxSource)
        .expect("TSX source probe should exist");

    assert_eq!(ts_source.rel_path, "src/index.ts");
    assert_eq!(tsx_source.rel_path, "src/app/page.tsx");
}
