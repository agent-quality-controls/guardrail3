#[test]
fn style_source_globs_are_scoped_to_app_root() {
    let policy = g3ts_style_types::G3TsStylePolicySurfaceState::Parsed {
        snapshot: g3ts_style_types::G3TsStylePolicySnapshot {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
            source_globs: vec!["src/**/*.{astro,ts,tsx}".to_owned()],
            stylelint_css_globs: Vec::new(),
            extra_fields: Vec::new(),
        },
    };

    g3ts_style_ingestion_assertions::eslint_directives::assert_scoped_style_globs(
        super::super::style_source_globs("apps/landing", &policy),
        &["apps/landing/src/**/*.{astro,ts,tsx}"],
    );
}

#[test]
fn compiled_style_source_globs_match_configured_files() {
    let globs = vec!["apps/landing/src/**/*.{astro,ts,tsx}".to_owned()];
    let compiled = super::super::compile_globs(&globs)
        .expect("test glob should compile");

    g3ts_style_ingestion_assertions::eslint_directives::assert_glob_matches(
        &compiled,
        "apps/landing/src/ui/page.tsx",
    );
    g3ts_style_ingestion_assertions::eslint_directives::assert_glob_does_not_match(
        &compiled,
        "apps/web/src/ui/page.tsx",
    );
}

#[test]
fn invalid_style_source_globs_fail_closed_for_disable_inventory() {
    let policy = g3ts_style_types::G3TsStylePolicySurfaceState::Parsed {
        snapshot: g3ts_style_types::G3TsStylePolicySnapshot {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
            source_globs: vec!["src/**/[".to_owned()],
            stylelint_css_globs: Vec::new(),
            extra_fields: Vec::new(),
        },
    };
    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: std::path::PathBuf::from("/tmp/g3ts-style-test"),
        entries: Vec::new(),
    };

    let directives = super::super::eslint_directives(&crawl, "apps/landing", &policy);

    assert_eq!(directives.len(), 1);
    let directive = directives
        .first()
        .expect("invalid glob should emit one fail-closed directive input");
    assert!(
        directive.parse_error.is_some(),
        "invalid style source glob should become a parse error input"
    );
}
