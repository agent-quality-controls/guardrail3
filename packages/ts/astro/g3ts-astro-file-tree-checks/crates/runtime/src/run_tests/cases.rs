use g3ts_astro_file_tree_checks_assertions::run as assertions;
use g3ts_astro_types::{
    G3TsAstroAppRootInput, G3TsAstroFileTreeChecksInput, G3TsAstroRouteMarkdownPageInput,
};

#[test]
fn golden_filetree_reports_expected_inventory() {
    let input = golden_build_collections_input();
    let results = crate::run::check(&input);

    assertions::assert_has_inventory(&results, "TS-ASTRO-SETUP-FILETREE-01");
    assertions::assert_has_inventory(&results, "TS-ASTRO-CONTENT-FILETREE-02");
    assertions::assert_missing(&results, "TS-ASTRO-SETUP-FILETREE-03");
    assertions::assert_missing(&results, "TS-ASTRO-CONTENT-FILETREE-04");
    assertions::assert_missing(&results, "TS-ASTRO-CONTENT-FILETREE-05");
    assertions::assert_missing(&results, "TS-ASTRO-CONTENT-FILETREE-06");
    assertions::assert_missing(&results, "TS-ASTRO-STATE-FILETREE-11");
    assertions::assert_missing(&results, "TS-ASTRO-STATE-FILETREE-12");
}

#[test]
fn child_filetree_entrypoints_report_exact_ids_in_order() {
    let input = golden_build_collections_input();
    assertions::assert_exact_ids(
        &crate::run::check_setup(&input),
        &["TS-ASTRO-SETUP-FILETREE-01"],
    );
    assertions::assert_exact_ids(
        &crate::run::check_content(&input),
        &["TS-ASTRO-CONTENT-FILETREE-02"],
    );
    assertions::assert_exact_ids(&crate::run::check_state(&input), &[]);
}

#[test]
fn child_filetree_entrypoints_own_error_rules() {
    let mut missing_live = golden_build_collections_input();
    missing_live.app_roots[0].live_config_rel_path = None;
    missing_live.live_collection_roots = vec![missing_live.app_roots[0].clone()];
    assertions::assert_has_error(
        &crate::run::check_setup(&missing_live),
        "TS-ASTRO-SETUP-FILETREE-03",
    );

    let mut route_markdown = golden_build_collections_input();
    route_markdown
        .route_markdown_pages
        .push(G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        });
    assertions::assert_has_error(
        &crate::run::check_content(&route_markdown),
        "TS-ASTRO-CONTENT-FILETREE-04",
    );

    let mut velite = golden_build_collections_input();
    velite.app_roots[0].velite_config_rel_path = Some("velite.config.mjs".to_owned());
    velite.app_roots[0].velite_output_rel_paths = vec![".velite/landing.js".to_owned()];
    velite.build_collection_roots[0] = velite.app_roots[0].clone();
    let content_results = crate::run::check_content(&velite);
    assertions::assert_has_error(&content_results, "TS-ASTRO-CONTENT-FILETREE-05");
    assertions::assert_has_error(&content_results, "TS-ASTRO-CONTENT-FILETREE-06");

    let mut state = golden_build_collections_input();
    state.app_roots[0].legacy_generated_state_rel_paths =
        vec![".next/server/app/page.js".to_owned()];
    state.app_roots[0].forbidden_state_rel_paths = vec![".velite/landing.js".to_owned()];
    state.build_collection_roots[0] = state.app_roots[0].clone();
    let state_results = crate::run::check_state(&state);
    assertions::assert_has_error(&state_results, "TS-ASTRO-STATE-FILETREE-11");
    assertions::assert_has_error(&state_results, "TS-ASTRO-STATE-FILETREE-12");
}

#[test]
fn missing_astro_config_reports_error() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: None,
            content_config_rel_path: Some("src/content.config.ts".to_owned()),
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-SETUP-FILETREE-01");
}

#[test]
fn missing_content_config_reports_error_for_build_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-02");
}

#[test]
fn route_markdown_pages_are_forbidden_for_build_collection_apps() {
    let mut input = golden_build_collections_input();
    input
        .route_markdown_pages
        .push(G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        });

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-04");
}

#[test]
fn missing_live_config_reports_error_for_live_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-SETUP-FILETREE-03");
}

#[test]
fn route_markdown_pages_do_not_fire_without_build_collections() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: vec![G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        }],
    };

    let results = crate::run::check(&input);
    assertions::assert_missing(&results, "TS-ASTRO-CONTENT-FILETREE-04");
}

#[test]
fn route_markdown_pages_are_forbidden_for_live_collection_apps() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: Some("src/live.config.ts".to_owned()),
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: Some("src/live.config.ts".to_owned()),
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        route_markdown_pages: vec![G3TsAstroRouteMarkdownPageInput {
            rel_path: "src/pages/about.mdx".to_owned(),
        }],
    };

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-04");
}

#[test]
fn velite_config_is_forbidden_for_astro_apps() {
    let mut input = golden_build_collections_input();
    input.app_roots[0].velite_config_rel_path = Some("velite.config.mjs".to_owned());
    input.build_collection_roots[0].velite_config_rel_path = Some("velite.config.mjs".to_owned());

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-05");
}

#[test]
fn velite_output_is_forbidden_for_astro_apps() {
    let mut input = golden_build_collections_input();
    input.app_roots[0].velite_output_rel_paths = vec![".velite/landing.js".to_owned()];
    input.build_collection_roots[0].velite_output_rel_paths = vec![".velite/landing.js".to_owned()];

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-06");
}

#[test]
fn velite_output_rule_only_applies_to_content_apps() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: vec![".velite/landing.js".to_owned()],
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_missing(&results, "TS-ASTRO-CONTENT-FILETREE-06");
}

#[test]
fn legacy_generated_state_is_forbidden_for_astro_apps() {
    let mut input = golden_build_collections_input();
    input.app_roots[0].legacy_generated_state_rel_paths = vec![
        ".next/server/app/page.js".to_owned(),
        ".contentlayer/generated/Post.mjs".to_owned(),
        "contentlayer.config.ts".to_owned(),
    ];
    input.build_collection_roots[0].legacy_generated_state_rel_paths =
        input.app_roots[0].legacy_generated_state_rel_paths.clone();

    let results = crate::run::check(&input);
    assertions::assert_error_files(
        &results,
        "TS-ASTRO-STATE-FILETREE-11",
        &[
            ".next/server/app/page.js",
            ".contentlayer/generated/Post.mjs",
            "contentlayer.config.ts",
        ],
    );
}

#[test]
fn legacy_generated_state_rule_only_applies_to_content_apps() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: vec![".next/server/app/page.js".to_owned()],
            forbidden_state_rel_paths: Vec::new(),
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_missing(&results, "TS-ASTRO-STATE-FILETREE-11");
}

#[test]
fn velite_and_legacy_state_can_report_together() {
    let mut input = golden_build_collections_input();
    input.app_roots[0].velite_output_rel_paths = vec![".velite/landing.js".to_owned()];
    input.app_roots[0].legacy_generated_state_rel_paths =
        vec![".next/server/app/page.js".to_owned()];
    input.build_collection_roots[0] = input.app_roots[0].clone();

    let results = crate::run::check(&input);
    assertions::assert_has_error(&results, "TS-ASTRO-CONTENT-FILETREE-06");
    assertions::assert_has_error(&results, "TS-ASTRO-STATE-FILETREE-11");
}

#[test]
fn configured_forbidden_state_is_forbidden_for_astro_content_apps() {
    let mut input = golden_build_collections_input();
    input.app_roots[0].forbidden_state_rel_paths = vec![
        ".next/server/app/page.js".to_owned(),
        ".velite/landing.js".to_owned(),
    ];
    input.build_collection_roots[0].forbidden_state_rel_paths =
        input.app_roots[0].forbidden_state_rel_paths.clone();

    let results = crate::run::check(&input);
    assertions::assert_error_files(
        &results,
        "TS-ASTRO-STATE-FILETREE-12",
        &[".next/server/app/page.js", ".velite/landing.js"],
    );
}

#[test]
fn configured_forbidden_state_rule_only_applies_to_content_apps() {
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: None,
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: vec![".next/server/app/page.js".to_owned()],
        }],
        build_collection_roots: Vec::new(),
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    let results = crate::run::check(&input);
    assertions::assert_missing(&results, "TS-ASTRO-STATE-FILETREE-12");
}

fn golden_build_collections_input() -> G3TsAstroFileTreeChecksInput {
    let root = G3TsAstroAppRootInput {
        app_root_rel_path: ".".to_owned(),
        astro_config_rel_path: Some("astro.config.mjs".to_owned()),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
        velite_config_rel_path: None,
        velite_output_rel_paths: Vec::new(),
        legacy_generated_state_rel_paths: Vec::new(),
        forbidden_state_rel_paths: Vec::new(),
    };

    G3TsAstroFileTreeChecksInput {
        app_roots: vec![root.clone()],
        build_collection_roots: vec![root],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    }
}
