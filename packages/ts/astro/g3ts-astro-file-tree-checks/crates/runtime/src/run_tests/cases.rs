use g3ts_astro_file_tree_checks_assertions::run as assertions;
use g3ts_astro_types::{
    G3TsAstroAppRootInput, G3TsAstroFileTreeChecksInput, G3TsAstroRouteMarkdownPageInput,
};

#[test]
fn golden_filetree_reports_expected_inventory() {
    let input = golden_build_collections_input();
    let results = crate::run::check(&input);

    assertions::assert_has_inventory(&results, "TS-ASTRO-FILETREE-01");
    assertions::assert_has_inventory(&results, "TS-ASTRO-FILETREE-02");
    assertions::assert_missing(&results, "TS-ASTRO-FILETREE-03");
    assertions::assert_missing(&results, "TS-ASTRO-FILETREE-04");
    assertions::assert_missing(&results, "TS-ASTRO-FILETREE-05");
}

#[test]
fn missing_astro_config_reports_error() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: None,
            content_config_rel_path: Some("src/content.config.ts".to_owned()),
            live_config_rel_path: None,
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
        cross_root_side_loaders: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-FILETREE-01");
}

#[test]
fn missing_content_config_reports_error_for_build_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
        }],
        build_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
        }],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
        cross_root_side_loaders: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-FILETREE-02");
}

#[test]
fn route_markdown_pages_are_forbidden_for_build_collection_apps() {
    let mut input = golden_build_collections_input();
    input.route_markdown_pages.push(G3TsAstroRouteMarkdownPageInput {
        rel_path: "src/pages/about.mdx".to_owned(),
    });

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-FILETREE-04");
}

#[test]
fn missing_live_config_reports_error_for_live_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
        }],
        route_markdown_pages: Vec::new(),
        cross_root_side_loaders: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-FILETREE-03");
}

#[test]
fn route_markdown_pages_do_not_fire_without_build_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: vec![G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        }],
        cross_root_side_loaders: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_missing(&results, "TS-ASTRO-FILETREE-04");
}

#[test]
fn route_markdown_pages_are_forbidden_for_live_collection_apps() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: Some("src/live.config.ts".to_owned()),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: Some("src/live.config.ts".to_owned()),
        }],
        route_markdown_pages: vec![G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        }],
        cross_root_side_loaders: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-FILETREE-04");
}

fn golden_build_collections_input() -> G3TsAstroFileTreeChecksInput {
    let root = G3TsAstroAppRootInput {
        app_root_rel_path: ".".to_owned(),
        astro_config_rel_path: Some("astro.config.mjs".to_owned()),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
    };

    G3TsAstroFileTreeChecksInput {
        app_roots: vec![root.clone()],
        build_collection_roots: vec![root],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
        cross_root_side_loaders: Vec::new(),
    }
}
