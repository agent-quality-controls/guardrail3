use g3ts_astro_config_checks_assertions::run as assertions;
use g3ts_astro_types::{
    G3TsAstroConfigChecksInput, G3TsAstroConfigSurfaceState, G3TsAstroContentMode,
    G3TsAstroEslintSurfaceState, G3TsAstroOutputMode, G3TsAstroPackageSurfaceState,
    G3TsAstroPolicySurfaceState, G3TsAstroStaticValue,
};

use super::helpers::{
    astro_check_wrapper_forms, astro_lane_missing_pipeline_effectiveness,
    endpoint_only_pipeline_scope_options, endpoint_only_pipeline_scope_without_route_coverage,
    fake_astro_check_text_only, golden, local_syncpack_package_source_covers_nested_app,
    malformed_syncpack_config, missing_astro_check, missing_astro_plugin_wiring,
    missing_content_data_module_scope_options, missing_content_source_scope_options,
    missing_inline_public_content_rule, missing_package_eslint_and_astro_config_surfaces,
    missing_pipeline_rule_enforcement, missing_pipeline_scope_options, missing_pipeline_wiring,
    missing_required_packages, missing_syncpack_config,
    root_syncpack_exact_source_covers_nested_app,
    root_syncpack_package_source_does_not_cover_nested_app, route_only_pipeline_wiring,
    syncpack_catch_all_forbidden_ban, syncpack_ignored_forbidden_ban,
    syncpack_missing_astro_seo_ban, syncpack_missing_contentlayer_ban,
    syncpack_missing_forbidden_ban, syncpack_missing_forbidden_ban_named,
    syncpack_missing_stack_pin, syncpack_pinned_forbidden_ban, syncpack_scoped_away_forbidden_ban,
    syncpack_scoped_away_stack_pin, syncpack_shadowed_forbidden_ban, syncpack_shadowed_stack_pin,
    syncpack_source_excludes_package, syncpack_specifier_scoped_forbidden_ban,
    syncpack_specifier_scoped_stack_pin, syncpack_wrong_astro_pipeline_stack_pin,
    syncpack_wrong_forbidden_ban_dependency_types, syncpack_wrong_stack_pin,
    ts_lane_missing_pipeline_effectiveness, tsx_lane_missing_pipeline_effectiveness,
    unreadable_syncpack_config, velite_package_with_syncpack_ban,
};

const PIPELINE_CONTENT_INFO_TITLE: &str = "Astro pipeline ESLint plugin is wired and effective";
const PIPELINE_CONTENT_ERROR_TITLE: &str =
    "Astro ESLint lanes are not enforcing the required content rules";
const PIPELINE_CONTENT_INFO_MJS: &str = "`eslint.config.mjs` activates `astro-pipeline` from `g3ts-eslint-plugin-astro-pipeline` and enforces the required Astro pipeline rules at error severity on the Astro, TS, and TSX source probes. Route-scoped rules cover actual page routes and endpoints; the content-data rule has non-empty `contentDataModuleGlobs`; the authored-content rules have non-empty `authoredContentGlobs` or `specContentGlobs`; and `astro-pipeline/require-approved-content-adapter-in-routes` has non-empty `approvedContentAdapterModules`.";
const PIPELINE_CONTENT_ERROR_MJS: &str = "`eslint.config.mjs` does not activate `astro-pipeline` from `g3ts-eslint-plugin-astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source probes. Import `g3ts-eslint-plugin-astro-pipeline`, register it as `astro-pipeline`, and enable the required Astro pipeline rules, route coverage for actual Astro page routes, endpoint coverage for actual Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, non-empty `authoredContentGlobs` or `specContentGlobs` on the authored-content rules, and non-empty `approvedContentAdapterModules` on `astro-pipeline/require-approved-content-adapter-in-routes`. Without those effective delegated rules, routes can bypass Astro content collections while the package is still installed.";
const PIPELINE_CONTENT_ERROR_GLOB: &str = "`eslint.config.*` does not activate `astro-pipeline` from `g3ts-eslint-plugin-astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source probes. Import `g3ts-eslint-plugin-astro-pipeline`, register it as `astro-pipeline`, and enable the required Astro pipeline rules, route coverage for actual Astro page routes, endpoint coverage for actual Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, non-empty `authoredContentGlobs` or `specContentGlobs` on the authored-content rules, and non-empty `approvedContentAdapterModules` on `astro-pipeline/require-approved-content-adapter-in-routes`. Without those effective delegated rules, routes can bypass Astro content collections while the package is still installed.";
const ASTRO_SEO_BAN_REASON: &str = "`astro-seo` is forbidden because `astro-seo@1.1.0` exports TypeScript source directly from the package entry point. Astro apps must use the approved SEO path instead: typed content/layout data, `schema-dts` for JSON-LD types, `@nuasite/checks` with `g3ts-astro-nuasite-checks` for rendered-output verification, `@astrojs/sitemap`, and `astro-robots`.";

#[test]
fn golden_config_reports_expected_inventory() {
    let input = golden();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`.syncpackrc` pins the required Syncpack package policy: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`.syncpackrc` bans forbidden Astro deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-01",
                "astro package present",
                "`package.json` includes `astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-02",
                "astro check present",
                "`package.json` installs `@astrojs/check` and safely invokes `astro check` in the app script surface.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-03",
                "astro ESLint plugin package present",
                "`package.json` includes `eslint-plugin-astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-05",
                "astro ESLint plugin wired",
                "`eslint.config.mjs` activates `astro` for the required Astro source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-06",
                "astro pipeline ESLint plugin package present",
                "`package.json` includes `g3ts-eslint-plugin-astro-pipeline`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-07",
                PIPELINE_CONTENT_INFO_TITLE,
                PIPELINE_CONTENT_INFO_MJS,
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-09",
                "Syncpack pins the required Astro stack",
                &pins_message,
                Some(".syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-10",
                "Syncpack bans forbidden Astro deps",
                &bans_message,
                Some(".syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-11",
                "Astro config has HTTPS site URL",
                "`astro.config.mjs` sets an absolute HTTPS `site` URL.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-12",
                "Astro config uses static output",
                "`astro.config.mjs` sets `output: \"static\"`.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-13",
                "Nuasite rendered-output checks are installed and wired",
                "`astro.config.mjs` wires `checks()` from `@nuasite/checks` with fail-closed options and the package scripts safely run `astro build`.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-14",
                "Astro sitemap integration is installed and wired",
                "`astro.config.mjs` wires default `sitemap()` from `@astrojs/sitemap` and has an HTTPS `site`.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-15",
                "Astro robots integration is installed and wired",
                "`astro.config.mjs` wires default `robots()` from `astro-robots` and has an HTTPS `site`.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-16",
                "LLMs discovery file exists",
                "Found `public/llms.txt`.",
                Some("public/llms.txt"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-17",
                "Astro JSON-LD type package is present",
                "`package.json` lists `schema-dts` for typed JSON-LD data.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-18",
                "Astro content adapter route rule is effective",
                "`eslint.config.mjs` enforces `astro-pipeline/require-approved-content-adapter-in-routes` from `g3ts-eslint-plugin-astro-pipeline` with route coverage, endpoint coverage, and non-empty `approvedContentAdapterModules` on Astro, TS, and TSX source probes.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-19",
                "Inline public-copy ESLint rule is effective",
                "`eslint.config.mjs` enforces `i18next/no-literal-string` with the exact strict Astro public-copy options on Astro, TS, and TSX source probes.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-20",
                "MDX ESLint lane is wired",
                "`eslint.config.mjs` activates plugin `mdx` and `mdx/remark` at error severity for the MDX content probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-21",
                "Required Astro integrations are present",
                "`astro.config.mjs` wires React, MDX, sitemap, robots, and Nuasite checks integrations from the approved packages.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-22",
                "JSON-LD presence check is delegated to Nuasite",
                "`astro.config.mjs` wires `structuredDataPresentCheck` imported from `g3ts-astro-nuasite-checks` through `checks({ customChecks: [...] })`.",
                Some("astro.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-23",
                "Astro strict content policy is configured",
                "`guardrail3-ts.toml` sets `[ts.astro] profile = \"strict-local-content\"`, declares non-empty `content_routes`, `content_root`, `content_adapter`, `mdx_component_maps`, `metadata_helpers`, and `json_ld_helpers`, and forbids `.next/**`, `.velite/**`, and `.contentlayer/**` generated state.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-24",
                "Astro strict content policy paths are structurally valid",
                "`guardrail3-ts.toml` uses app-relative `content_routes`, `non_content_routes`, `endpoints`, `content_root`, `content_adapter`, `mdx_component_maps`, `metadata_helpers`, `json_ld_helpers`, and `forbidden_state` values without parent traversal.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-25",
                "Astro content and non-content route scopes are disjoint",
                "`guardrail3-ts.toml` classifies discovered route pages without overlap between `content_routes` and `non_content_routes`.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-26",
                "Astro ESLint route coverage matches strict content policy",
                "`guardrail3-ts.toml` and `eslint.config.mjs` agree on content route, non-content route, and endpoint coverage for the required Astro pipeline rules.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-27",
                "Astro content adapter source exists",
                "`guardrail3-ts.toml` resolves `content_adapter = \"src/lib/content\"` to adapter source files: `src/lib/content/index.ts`.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-28",
                "Astro content adapter sources import Astro content collections",
                "`guardrail3-ts.toml` resolves `content_adapter` to adapter source files that import `astro:content` at runtime: `src/lib/content/index.ts`.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-29",
                "Astro strict content policy declares approved helper surfaces",
                "`guardrail3-ts.toml` declares non-empty app-relative `mdx_component_maps`, `metadata_helpers`, and `json_ld_helpers` in `[ts.astro]`, and those helper surfaces do not overlap `content_root`.",
                Some("guardrail3-ts.toml"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-30",
                "Astro MDX component-map rule is effective",
                "`eslint.config.mjs` enforces `astro-pipeline/mdx-component-imports-from-approved-map` from `g3ts-eslint-plugin-astro-pipeline` on the MDX content lane with non-empty `mdxContentGlobs` and `approvedMdxComponentModules`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-31",
                "Astro metadata helper rule is effective",
                "`eslint.config.mjs` enforces `astro-pipeline/require-approved-metadata-helper-in-routes` from `g3ts-eslint-plugin-astro-pipeline` on Astro, TS, and TSX lanes with route coverage, endpoint coverage, non-empty `approvedMetadataHelperModules`, and non-empty `approvedContentAdapterModules`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-32",
                "Astro JSON-LD helper rule is effective",
                "`eslint.config.mjs` enforces `astro-pipeline/require-approved-json-ld-helper-in-routes` from `g3ts-eslint-plugin-astro-pipeline` on Astro, TS, and TSX lanes with route coverage, endpoint coverage, and non-empty `approvedJsonLdHelperModules`.",
                Some("eslint.config.mjs"),
                true,
            ),
        ],
    );
}

#[test]
fn strict_content_policy_rule_rejects_missing_policy() {
    let mut input = golden();
    input.integration_contracts[0].astro_policy = G3TsAstroPolicySurfaceState::MissingAstroPolicy {
        rel_path: "guardrail3-ts.toml".to_owned(),
    };

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-23",
        "Astro strict content policy is missing or incomplete",
    );
}

#[test]
fn strict_content_policy_rule_rejects_old_route_class_policy() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_routes.clear();

    let results = super::super::check(&input);

    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-23",
        "old `*_globs` route-class fields are not supported",
    );
}

#[test]
fn strict_content_policy_path_rule_rejects_absolute_parent_and_backslash_paths() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_routes = vec!["/src/pages/**/*.astro".to_owned()];
    snapshot.non_content_routes = vec!["../src/pages/404.astro".to_owned()];
    snapshot.endpoints = vec!["src\\pages\\api.ts".to_owned()];
    snapshot.forbidden_state = vec![".next/**".to_owned(), "../.velite/**".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-24",
        "Astro strict content policy paths are invalid",
    );
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-24", "content_routes");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-24", "non_content_routes");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-24", "endpoints");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-24", "forbidden_state");
}

#[test]
fn strict_content_policy_path_rule_rejects_glob_dirs_and_overlapping_roots() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_root = Some("src/content".to_owned());
    snapshot.content_adapter = Some("src/content/adapters".to_owned());

    let results = super::super::check(&input);

    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-24",
        "content_root overlaps content_adapter",
    );

    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_adapter = Some("src/lib/content/**".to_owned());

    let results = super::super::check(&input);

    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-24", "content_adapter");
}

#[test]
fn strict_content_policy_path_rule_rejects_helper_surfaces_under_content_root() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_root = Some("src/content".to_owned());
    snapshot.mdx_component_maps = vec!["src/content/mdx-components".to_owned()];
    snapshot.metadata_helpers = vec!["src/content/metadata".to_owned()];
    snapshot.json_ld_helpers = vec!["src/content/json-ld".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-24",
        "content_root overlaps mdx_component_maps",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-24",
        "content_root overlaps metadata_helpers",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-24",
        "content_root overlaps json_ld_helpers",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-29",
        "mdx_component_maps overlaps content_root",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-29",
        "metadata_helpers overlaps content_root",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-29",
        "json_ld_helpers overlaps content_root",
    );
}

#[test]
fn strict_content_policy_route_scope_rule_rejects_discovered_overlap() {
    let mut input = golden();
    input.integration_contracts[0]
        .route_page_paths
        .push("src/pages/404.astro".to_owned());
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_routes = vec!["src/pages/**/*.astro".to_owned()];
    snapshot.non_content_routes = vec!["src/pages/404.astro".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-25",
        "Astro content and non-content route scopes overlap",
    );
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-25", "src/pages/404.astro");
}

#[test]
fn strict_content_policy_route_scope_rule_rejects_invalid_globs() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_routes = vec!["src/pages/[".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-25",
        "Astro route scope policy contains an invalid glob",
    );
}

#[test]
fn strict_content_policy_eslint_coverage_rule_rejects_non_content_route_coverage() {
    let mut input = golden();
    input.integration_contracts[0]
        .route_page_paths
        .push("src/pages/404.astro".to_owned());
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.content_routes = vec!["src/pages/index.astro".to_owned()];
    snapshot.non_content_routes = vec!["src/pages/404.astro".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-26",
        "Astro ESLint route coverage does not match strict content policy",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-26",
        "exclude `[ts.astro].non_content_routes`",
    );
}

#[test]
fn strict_content_policy_eslint_coverage_rule_rejects_missing_content_route_coverage() {
    let mut input = golden();
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint surface should be parsed");
    };
    for scope in &mut snapshot.astro_source_route_scoped_pipeline_rule_scopes {
        scope.route_globs = vec!["src/pages/other.astro".to_owned()];
    }

    let results = super::super::check(&input);

    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-26",
        "Astro lane `astro-pipeline/",
    );
}

#[test]
fn strict_content_policy_adapter_rule_rejects_missing_adapter_source() {
    let mut input = golden();
    input.integration_contracts[0]
        .approved_surface_sources.content_adapter
        .clear();

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-27",
        "Astro content adapter source is missing",
    );
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-27", "src/lib/content");
}

#[test]
fn strict_content_policy_adapter_rule_rejects_adapter_source_without_astro_content_import() {
    let mut input = golden();
    input.integration_contracts[0]
        .approved_surface_sources.content_adapter_astro_content
        .clear();

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-28",
        "Astro content adapter source does not use Astro content collections",
    );
    assertions::assert_id_message_contains(
        &results,
        "TS-ASTRO-CONFIG-28",
        "src/lib/content/index.ts",
    );
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-28", "astro:content");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-28", "Type-only imports");
}

#[test]
fn strict_content_policy_helper_surface_rule_rejects_missing_policy_fields() {
    let mut input = golden();
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_policy
    else {
        panic!("golden astro policy should be parsed");
    };
    snapshot.mdx_component_maps.clear();
    snapshot.metadata_helpers.clear();
    snapshot.json_ld_helpers.clear();

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-29",
        "Astro strict content policy is missing approved helper surfaces",
    );
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-29", "mdx_component_maps");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-29", "metadata_helpers");
    assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-29", "json_ld_helpers");
}

#[test]
fn strict_content_policy_helper_rules_reject_missing_source_files() {
    let mut input = golden();
    input.integration_contracts[0]
        .approved_surface_sources
        .missing_mdx_component_maps
        .push("src/components/missing-mdx".to_owned());
    input.integration_contracts[0]
        .approved_surface_sources
        .missing_metadata_helpers
        .push("src/lib/missing-metadata".to_owned());
    input.integration_contracts[0]
        .approved_surface_sources
        .missing_json_ld_helpers
        .push("src/lib/missing-json-ld".to_owned());

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-30",
        "Astro MDX component-map sources are missing",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-31",
        "Astro metadata helper sources are missing",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-32",
        "Astro JSON-LD helper sources are missing",
    );
}

#[test]
fn strict_content_policy_helper_rules_reject_missing_eslint_enforcement() {
    let mut input = golden();
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint surface should be parsed");
    };
    snapshot
        .mdx_content_effective_mdx_component_map_rules
        .clear();
    snapshot
        .astro_source_effective_metadata_helper_rules
        .clear();
    snapshot.ts_source_effective_metadata_helper_rules.clear();
    snapshot.tsx_source_effective_metadata_helper_rules.clear();
    snapshot.astro_source_effective_json_ld_helper_rules.clear();
    snapshot.ts_source_effective_json_ld_helper_rules.clear();
    snapshot.tsx_source_effective_json_ld_helper_rules.clear();

    let results = super::super::check(&input);

    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-30",
        "Astro MDX component-map rule is not effective",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-31",
        "Astro metadata helper rule is not effective",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-32",
        "Astro JSON-LD helper rule is not effective",
    );
}

#[test]
fn astro_config_site_url_rule_rejects_missing_or_non_https_site() {
    for site in [None, Some("http://example.com".to_owned())] {
        let mut input = golden();
        let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
            &mut input.integration_contracts[0].astro_config
        else {
            panic!("golden astro config should be parsed");
        };
        snapshot.site = site;

        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-11",
            "Astro config is missing an absolute HTTPS `site` URL",
        );
    }
}

#[test]
fn astro_static_output_rule_rejects_missing_or_server_output() {
    for output in [None, Some(G3TsAstroOutputMode::Server)] {
        let mut input = golden();
        let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
            &mut input.integration_contracts[0].astro_config
        else {
            panic!("golden astro config should be parsed");
        };
        snapshot.output = output;

        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-12",
            "Astro public content app must use explicit static output",
        );
    }
}

#[test]
fn astro_check_rule_requires_package_and_safe_script() {
    let mut input = golden();
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        panic!("golden package should be parsed");
    };
    snapshot
        .dev_dependencies
        .retain(|dependency| dependency != "@astrojs/check");

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-02",
        "Astro app typecheck contract is missing",
    );
}

#[test]
fn nuasite_rule_rejects_unsafe_build_and_fail_open_options() {
    let mut unsafe_build = golden();
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut unsafe_build.integration_contracts[0].package
    else {
        panic!("golden package should be parsed");
    };
    snapshot.safely_runs_astro_build = false;
    let results = super::super::check(&unsafe_build);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-13",
        "Nuasite rendered-output checks are not installed and wired",
    );

    for (key, value) in [
        ("mode", G3TsAstroStaticValue::String("basic".to_owned())),
        ("failOnError", G3TsAstroStaticValue::Bool(false)),
        ("failOnWarning", G3TsAstroStaticValue::Bool(false)),
        ("reportJson", G3TsAstroStaticValue::Bool(false)),
        ("ai", G3TsAstroStaticValue::Bool(true)),
        ("seo", G3TsAstroStaticValue::Bool(false)),
        ("geo", G3TsAstroStaticValue::Bool(false)),
        ("performance", G3TsAstroStaticValue::Bool(false)),
        ("accessibility", G3TsAstroStaticValue::Bool(false)),
        ("customChecks", G3TsAstroStaticValue::Array(Vec::new())),
        (
            "overrides",
            G3TsAstroStaticValue::Object(vec![g3ts_astro_types::G3TsAstroStaticObjectProperty {
                key: "anything".to_owned(),
                value: G3TsAstroStaticValue::Bool(true),
            }]),
        ),
    ] {
        let mut input = golden();
        mutate_nuasite_option(&mut input, key, value);
        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-13",
            "Nuasite rendered-output checks are not installed and wired",
        );
        if key == "customChecks" {
            assertions::assert_has_error_title(
                &results,
                "TS-ASTRO-CONFIG-22",
                "JSON-LD presence check is not delegated to Nuasite",
            );
        }
    }
}

#[test]
fn nuasite_rule_rejects_missing_required_options_and_duplicate_keys() {
    for key in [
        "mode",
        "failOnError",
        "failOnWarning",
        "reportJson",
        "ai",
        "customChecks",
    ] {
        let mut input = golden();
        remove_nuasite_option(&mut input, key);
        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-13",
            "Nuasite rendered-output checks are not installed and wired",
        );
        if key == "customChecks" {
            assertions::assert_has_error_title(
                &results,
                "TS-ASTRO-CONFIG-22",
                "JSON-LD presence check is not delegated to Nuasite",
            );
        }
    }

    let mut duplicate = golden();
    duplicate_nuasite_option(
        &mut duplicate,
        "mode",
        G3TsAstroStaticValue::String("full".to_owned()),
    );
    let results = super::super::check(&duplicate);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-13",
        "Nuasite rendered-output checks are not installed and wired",
    );

    let mut unknown = golden();
    mutate_nuasite_option(&mut unknown, "unreviewed", G3TsAstroStaticValue::Bool(true));
    let results = super::super::check(&unknown);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-13",
        "Nuasite rendered-output checks are not installed and wired",
    );
}

#[test]
fn astro_generation_rules_reject_missing_integration_surfaces() {
    let mut input = golden();
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    snapshot.integrations.retain(|integration| {
        !matches!(
            integration.source_module.as_deref(),
            Some("@astrojs/sitemap") | Some("astro-robots")
        )
    });

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-14",
        "Astro sitemap integration is not installed and wired",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-15",
        "Astro robots integration is not installed and wired",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-21",
        "Required Astro integrations are missing",
    );
}

#[test]
fn astro_generation_rules_reject_non_exact_integration_calls() {
    for module in [
        "@astrojs/react",
        "@astrojs/mdx",
        "@astrojs/sitemap",
        "astro-robots",
    ] {
        let mut input = golden();
        let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
            &mut input.integration_contracts[0].astro_config
        else {
            panic!("golden astro config should be parsed");
        };
        let integration = snapshot
            .integrations
            .iter_mut()
            .find(|integration| integration.source_module.as_deref() == Some(module))
            .expect("golden integration should exist");
        integration
            .call
            .as_mut()
            .expect("golden integration should be called")
            .first_arg = Some(G3TsAstroStaticValue::Object(Vec::new()));

        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-21",
            "Required Astro integrations are missing",
        );
        if module == "@astrojs/sitemap" {
            assertions::assert_has_error_title(
                &results,
                "TS-ASTRO-CONFIG-14",
                "Astro sitemap integration is not installed and wired",
            );
        }
        if module == "astro-robots" {
            assertions::assert_has_error_title(
                &results,
                "TS-ASTRO-CONFIG-15",
                "Astro robots integration is not installed and wired",
            );
        }
    }
}

#[test]
fn required_integrations_rule_rejects_wrapped_or_wrong_import_shapes() {
    for module in [
        "@astrojs/react",
        "@astrojs/mdx",
        "@astrojs/sitemap",
        "astro-robots",
    ] {
        let mut input = golden();
        let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
            &mut input.integration_contracts[0].astro_config
        else {
            panic!("golden astro config should be parsed");
        };
        let integration = snapshot
            .integrations
            .iter_mut()
            .find(|integration| integration.source_module.as_deref() == Some(module))
            .expect("golden integration should exist");
        integration.imported_name = Some("named".to_owned());

        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-21",
            "Required Astro integrations are missing",
        );
    }

    let mut wrapped = golden();
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut wrapped.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    snapshot.integrations[0].call = None;
    let results = super::super::check(&wrapped);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-21",
        "Required Astro integrations are missing",
    );
}

#[test]
fn required_integrations_rule_rejects_missing_react_mdx_or_nuasite_integrations() {
    for module in ["@astrojs/react", "@astrojs/mdx", "@nuasite/checks"] {
        let mut input = golden();
        let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
            &mut input.integration_contracts[0].astro_config
        else {
            panic!("golden astro config should be parsed");
        };
        snapshot
            .integrations
            .retain(|integration| integration.source_module.as_deref() != Some(module));

        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-21",
            "Required Astro integrations are missing",
        );
    }
}

#[test]
fn required_integrations_rule_rejects_nuasite_call_without_options_object() {
    let mut input = golden();
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let integration = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("golden Nuasite integration should exist");
    integration
        .call
        .as_mut()
        .expect("golden Nuasite integration should be called")
        .first_arg = Some(G3TsAstroStaticValue::Bool(false));

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-21",
        "Required Astro integrations are missing",
    );
}

#[test]
fn content_discovery_and_seo_package_rules_reject_missing_surfaces() {
    let mut input = golden();
    input.integration_contracts[0].llms_txt_rel_path = None;
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        panic!("golden package should be parsed");
    };
    snapshot
        .dev_dependencies
        .retain(|dependency| dependency != "schema-dts");

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-16",
        "Astro public content app is missing `public/llms.txt`",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-17",
        "Astro JSON-LD type package is missing",
    );
    assertions::assert_no_id_message_contains(&results, "TS-ASTRO-CONFIG-17", "`astro-seo`");
}

#[test]
fn seo_package_rule_does_not_require_bare_astro_seo_package() {
    let input = golden();
    let results = super::super::check(&input);

    assertions::assert_no_id_message_contains(&results, "TS-ASTRO-CONFIG-17", "`astro-seo`");
}

#[test]
fn mdx_lane_rule_rejects_ignored_or_missing_mdx_probe() {
    let mut input = golden();
    let G3TsAstroConfigSurfaceState::Parsed { .. } = &input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint config should be parsed");
    };
    snapshot.mdx_content_probe_present = false;
    snapshot.mdx_content_probe_ignored = true;
    snapshot.mdx_content_plugins.clear();
    snapshot.mdx_content_error_rules.clear();

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-20",
        "MDX ESLint lane is not wired",
    );
}

#[test]
fn structured_data_rule_rejects_local_or_missing_custom_check() {
    for source_module in [None, Some("src/checks/structured-data".to_owned())] {
        let mut input = golden();
        replace_structured_data_check_source(&mut input, source_module);
        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-22",
            "JSON-LD presence check is not delegated to Nuasite",
        );
    }
}

#[test]
fn structured_data_rule_rejects_wrong_custom_check_identity_or_shape() {
    for (local_name, imported_name) in [
        ("otherCheck", Some("structuredDataPresentCheck".to_owned())),
        ("structuredDataPresentCheck", Some("otherCheck".to_owned())),
        ("structuredDataPresentCheck", None),
    ] {
        let mut input = golden();
        replace_structured_data_check_identity(&mut input, local_name, imported_name);
        let results = super::super::check(&input);
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-22",
            "JSON-LD presence check is not delegated to Nuasite",
        );
        assertions::assert_has_error_title(
            &results,
            "TS-ASTRO-CONFIG-13",
            "Nuasite rendered-output checks are not installed and wired",
        );
    }

    let mut non_array = golden();
    mutate_nuasite_option(
        &mut non_array,
        "customChecks",
        G3TsAstroStaticValue::Bool(true),
    );
    let results = super::super::check(&non_array);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-22",
        "JSON-LD presence check is not delegated to Nuasite",
    );
}

#[test]
fn pipeline_rules_reject_local_plugin_registered_under_astro_pipeline_namespace() {
    let mut input = golden();
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint config should be parsed");
    };
    let _ = snapshot
        .astro_source_plugin_package_names
        .remove("astro-pipeline");
    let _ = snapshot
        .ts_source_plugin_package_names
        .remove("astro-pipeline");
    let _ = snapshot
        .tsx_source_plugin_package_names
        .remove("astro-pipeline");

    let results = super::super::check(&input);
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-07",
        "Astro ESLint lanes are not enforcing the required content rules",
    );
    assertions::assert_has_error_title(
        &results,
        "TS-ASTRO-CONFIG-18",
        "Astro content adapter route rule is not effective",
    );
}

#[test]
fn delegated_eslint_rules_accept_effective_public_namespaces_without_package_identity() {
    let mut input = golden();
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint config should be parsed");
    };
    let _ = snapshot.astro_source_plugin_package_names.remove("i18next");
    let _ = snapshot.ts_source_plugin_package_names.remove("i18next");
    let _ = snapshot.tsx_source_plugin_package_names.remove("i18next");
    let _ = snapshot.mdx_content_plugin_package_names.remove("mdx");

    let results = super::super::check(&input);
    assertions::assert_has_info_title(
        &results,
        "TS-ASTRO-CONFIG-19",
        "Inline public-copy ESLint rule is effective",
    );
    assertions::assert_has_info_title(&results, "TS-ASTRO-CONFIG-20", "MDX ESLint lane is wired");
}

#[test]
fn astro_eslint_rule_accepts_effective_namespace_without_package_identity() {
    let mut input = golden();
    let G3TsAstroEslintSurfaceState::Parsed { snapshot } = &mut input.eslint_contracts[0].config
    else {
        panic!("golden eslint config should be parsed");
    };
    let _ = snapshot.astro_source_plugin_package_names.remove("astro");

    let results = super::super::check(&input);
    assertions::assert_has_info_title(&results, "TS-ASTRO-CONFIG-05", "astro ESLint plugin wired");
}

#[test]
fn syncpack_policy_rules_do_not_own_safe_syncpack_lint_script() {
    let mut input = golden();
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        panic!("golden package should be parsed");
    };
    snapshot.safely_runs_syncpack_lint = false;

    let results = super::super::check(&input);
    assertions::assert_has_info_title(
        &results,
        "TS-ASTRO-CONFIG-09",
        "Syncpack pins the required Astro stack",
    );
    assertions::assert_has_info_title(
        &results,
        "TS-ASTRO-CONFIG-10",
        "Syncpack bans forbidden Astro deps",
    );
}

#[test]
fn missing_syncpack_config_reports_stack_pin_and_ban_errors() {
    let input = missing_syncpack_config();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`.syncpackrc` is missing, so the Astro family cannot prove Syncpack pins the required Astro stack for `package.json`. Add a parseable `.syncpackrc` with canonical pinned `versionGroups` for: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`.syncpackrc` is missing, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `package.json`. Add a parseable `.syncpackrc` with canonical `isBanned: true` versionGroups for {}. {ASTRO_SEO_BAN_REASON}",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                &pins_message,
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro deps",
                &bans_message,
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn unavailable_syncpack_config_reports_unreadable_and_parse_error_reasons() {
    for (input, reason) in [
        (unreadable_syncpack_config(), "permission denied"),
        (
            malformed_syncpack_config(),
            "Syncpack config field `versionGroups` must be an array",
        ),
    ] {
        let results = super::super::check(&input);
        let pins_message = format!(
            "`.syncpackrc` {reason}, so the Astro family cannot prove Syncpack pins the required Astro stack for `package.json`. Add a parseable `.syncpackrc` with canonical pinned `versionGroups` for: {}.",
            crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
        );
        let bans_message = format!(
            "`.syncpackrc` {reason}, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `package.json`. Add a parseable `.syncpackrc` with canonical `isBanned: true` versionGroups for {}. {ASTRO_SEO_BAN_REASON}",
            crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
        );

        assertions::assert_contains(
            &results,
            &[
                assertions::error(
                    "TS-ASTRO-CONFIG-09",
                    "Syncpack does not pin the required Astro stack",
                    &pins_message,
                    Some(".syncpackrc"),
                    false,
                ),
                assertions::error(
                    "TS-ASTRO-CONFIG-10",
                    "Syncpack does not ban forbidden Astro deps",
                    &bans_message,
                    Some(".syncpackrc"),
                    false,
                ),
            ],
        );
    }
}

#[test]
fn syncpack_source_must_cover_the_app_package_manifest() {
    let input = syncpack_source_excludes_package();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                "`.syncpackrc` does not include exact `source` entry `package.json` for `package.json`, so `syncpack lint` cannot prove package policy for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro deps",
                &format!(
                    "`.syncpackrc` does not include exact `source` entry `package.json` for `package.json`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app. {ASTRO_SEO_BAN_REASON}"
                ),
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn root_syncpack_source_package_json_does_not_cover_nested_app_manifest() {
    let input = root_syncpack_package_source_does_not_cover_nested_app();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                "`.syncpackrc` does not include exact `source` entry `package.json` for `apps/landing/package.json`, so `syncpack lint` cannot prove package policy for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro deps",
                &format!(
                    "`.syncpackrc` does not include exact `source` entry `package.json` for `apps/landing/package.json`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app. {ASTRO_SEO_BAN_REASON}"
                ),
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn root_syncpack_source_exact_path_does_not_cover_nested_app_manifest() {
    let input = root_syncpack_exact_source_covers_nested_app();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                "`.syncpackrc` does not include exact `source` entry `package.json` for `apps/landing/package.json`, so `syncpack lint` cannot prove package policy for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro deps",
                &format!(
                    "`.syncpackrc` does not include exact `source` entry `package.json` for `apps/landing/package.json`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app. {ASTRO_SEO_BAN_REASON}"
                ),
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn app_local_syncpack_source_package_json_covers_nested_app_manifest() {
    let input = local_syncpack_package_source_covers_nested_app();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`apps/landing/.syncpackrc` pins the required Syncpack package policy: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`apps/landing/.syncpackrc` bans forbidden Astro deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_contains(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-09",
                "Syncpack pins the required Astro stack",
                &pins_message,
                Some("apps/landing/.syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-10",
                "Syncpack bans forbidden Astro deps",
                &bans_message,
                Some("apps/landing/.syncpackrc"),
                true,
            ),
        ],
    );
}

#[test]
fn missing_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_missing_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn wrong_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_wrong_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn wrong_astro_pipeline_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_wrong_astro_pipeline_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `g3ts-eslint-plugin-astro-pipeline` -> `0.1.6`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn shadowed_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_shadowed_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn package_scoped_away_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_scoped_away_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn specifier_scoped_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_specifier_scoped_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn syncpack_catch_all_forbidden_ban_does_not_satisfy_canonical_contract() {
    let input = syncpack_catch_all_forbidden_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`, `velite`, `@astrojs/node`, `eslint-plugin-astro-pipeline`, `@codemint/astro-meta`, `astro-seo`, `astro-seo-meta`, `astro-seo-schema`, `contentlayer`, `next-contentlayer`, `@contentlayer/core`, `@contentlayer/source-files`. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`. `astro-seo` is forbidden because `astro-seo@1.1.0` exports TypeScript source directly from the package entry point. Astro apps must use the approved SEO path instead: typed content/layout data, `schema-dts` for JSON-LD types, `@nuasite/checks` with `g3ts-astro-nuasite-checks` for rendered-output verification, `@astrojs/sitemap`, and `astro-robots`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn noncanonical_syncpack_forbidden_bans_report_policy_error() {
    for (_case_name, input) in [
        (
            "shadowed",
            syncpack_shadowed_forbidden_ban as fn() -> g3ts_astro_types::G3TsAstroConfigChecksInput,
        ),
        ("package scoped", syncpack_scoped_away_forbidden_ban),
        ("specifier scoped", syncpack_specifier_scoped_forbidden_ban),
        (
            "wrong dependency types",
            syncpack_wrong_forbidden_ban_dependency_types,
        ),
        ("ignored", syncpack_ignored_forbidden_ban),
        ("pinned", syncpack_pinned_forbidden_ban),
    ] {
        let results = super::super::check(&input());

        assertions::assert_contains(
            &results,
            &[assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro deps",
                "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`.",
                Some(".syncpackrc"),
                false,
            )],
        );
        assertions::assert_id_message_contains(&results, "TS-ASTRO-CONFIG-10", "`next`");
    }
}

#[test]
fn missing_syncpack_forbidden_ban_reports_policy_error() {
    let input = syncpack_missing_forbidden_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn missing_astro_seo_syncpack_ban_explains_approved_seo_path() {
    let input = syncpack_missing_astro_seo_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `astro-seo`. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`. `astro-seo` is forbidden because `astro-seo@1.1.0` exports TypeScript source directly from the package entry point. Astro apps must use the approved SEO path instead: typed content/layout data, `schema-dts` for JSON-LD types, `@nuasite/checks` with `g3ts-astro-nuasite-checks` for rendered-output verification, `@astrojs/sitemap`, and `astro-robots`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn missing_contentlayer_syncpack_ban_reports_policy_error() {
    let input = syncpack_missing_contentlayer_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `contentlayer`. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn all_contentlayer_syncpack_bans_are_required() {
    for dependency in [
        "contentlayer",
        "next-contentlayer",
        "@contentlayer/core",
        "@contentlayer/source-files",
    ] {
        let input = syncpack_missing_forbidden_ban_named(dependency);
        let results = super::super::check(&input);

        assertions::assert_id_message_contains(
            &results,
            "TS-ASTRO-CONFIG-10",
            &format!("`{dependency}`"),
        );
    }
}

#[test]
fn contentlayer_syncpack_bans_are_not_required_for_non_content_astro_apps() {
    let mut input = syncpack_missing_forbidden_ban_named("contentlayer");
    input.integration_contracts[0].content_mode = G3TsAstroContentMode::None;
    let results = super::super::check(&input);

    assertions::assert_no_id_message_contains(&results, "TS-ASTRO-CONFIG-10", "`contentlayer`");
}

#[test]
fn direct_velite_package_is_not_scanned_when_syncpack_ban_contract_is_valid() {
    let input = velite_package_with_syncpack_ban();
    let results = super::super::check(&input);
    let bans_message = format!(
        "`.syncpackrc` bans forbidden Astro deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-04");
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-10",
            "Syncpack bans forbidden Astro deps",
            &bans_message,
            Some(".syncpackrc"),
            true,
        )],
    );
}

#[test]
fn missing_astro_check_reports_only_that_error() {
    let input = missing_astro_check();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "Astro app typecheck contract is missing",
            "`package.json` violates the Astro typecheck contract: no app script safely invokes `astro check`. Install `@astrojs/check` and add a script that safely runs `astro check`. Text like `echo astro check`, `astro check || true`, or unsupported shell syntax does not satisfy this rule.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn quoted_astro_check_text_does_not_satisfy_the_script_contract() {
    let input = fake_astro_check_text_only();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "Astro app typecheck contract is missing",
            "`package.json` violates the Astro typecheck contract: no app script safely invokes `astro check`. Install `@astrojs/check` and add a script that safely runs `astro check`. Text like `echo astro check`, `astro check || true`, or unsupported shell syntax does not satisfy this rule.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn wrapper_forms_satisfy_the_astro_check_contract() {
    let input = astro_check_wrapper_forms();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-02",
            "astro check present",
            "`package.json` installs `@astrojs/check` and safely invokes `astro check` in the app script surface.",
            Some("package.json"),
            true,
        )],
    );
}

#[test]
fn missing_required_packages_report_package_contract_errors() {
    let input = missing_required_packages();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `g3ts-eslint-plugin-astro-pipeline`",
                "`package.json` does not list `g3ts-eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `g3ts-eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
        ],
    );
}

#[test]
fn missing_pipeline_wiring_reports_wiring_error() {
    let input = missing_astro_plugin_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-05",
            "Astro ESLint source probe is not wired to `eslint-plugin-astro`",
            "`eslint.config.mjs` does not activate `astro` from `eslint-plugin-astro` on the required Astro source probe. Add the `astro` plugin from `eslint-plugin-astro` to the Astro file lane in `eslint.config.mjs`. Astro files must run through the Astro plugin so framework lint rules actually execute.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_wiring_reports_pipeline_wiring_error() {
    let input = missing_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_rule_enforcement_reports_effectiveness_error() {
    let input = missing_pipeline_rule_enforcement();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_scope_options_reports_effectiveness_error() {
    let input = missing_pipeline_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_content_data_module_scope_options_reports_effectiveness_error() {
    let input = missing_content_data_module_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_content_source_scope_options_reports_effectiveness_error() {
    let input = missing_content_source_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_inline_public_content_rule_reports_effectiveness_error() {
    let input = missing_inline_public_content_rule();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-19",
            "Inline public-copy ESLint rule is not effective",
            "`eslint.config.mjs` must activate plugin `i18next` and rule `i18next/no-literal-string` at `error` on Astro, TS, and TSX source probes with the exact strict options from the Astro delegation plan. Missing probes, ignored probes, broad allowlists, or changed messages fail this contract.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn route_only_pipeline_wiring_still_fails_the_source_lane_contract() {
    let input = route_only_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn endpoint_only_scope_options_satisfy_pipeline_effectiveness() {
    let input = endpoint_only_pipeline_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_INFO_TITLE,
            PIPELINE_CONTENT_INFO_MJS,
            Some("eslint.config.mjs"),
            true,
        )],
    );
}

#[test]
fn endpoint_only_scope_options_fail_when_route_coverage_is_missing() {
    let input = endpoint_only_pipeline_scope_without_route_coverage();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_one_lane_still_fails_the_contract() {
    let input = tsx_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_the_astro_lane_still_fails_the_contract() {
    let input = astro_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_the_ts_lane_still_fails_the_contract() {
    let input = ts_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_package_eslint_and_astro_config_surfaces_fail_closed() {
    let input = missing_package_eslint_and_astro_config_surfaces();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-02",
                "Astro app typecheck contract is missing",
                "`package.json` violates the Astro typecheck contract: `@astrojs/check` is missing and no app script safely invokes `astro check`. Install `@astrojs/check` and add a script that safely runs `astro check`. Text like `echo astro check`, `astro check || true`, or unsupported shell syntax does not satisfy this rule.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-05",
                "Astro ESLint source probe is not wired to `eslint-plugin-astro`",
                "`eslint.config.*` does not activate `astro` from `eslint-plugin-astro` on the required Astro source probe. Add the `astro` plugin from `eslint-plugin-astro` to the Astro file lane in `eslint.config.*`. Astro files must run through the Astro plugin so framework lint rules actually execute.",
                Some("eslint.config.*"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `g3ts-eslint-plugin-astro-pipeline`",
                "`package.json` does not list `g3ts-eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `g3ts-eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-07",
                PIPELINE_CONTENT_ERROR_TITLE,
                PIPELINE_CONTENT_ERROR_GLOB,
                Some("eslint.config.*"),
                false,
            ),
        ],
    );
}

fn mutate_nuasite_option(
    input: &mut G3TsAstroConfigChecksInput,
    key: &str,
    value: G3TsAstroStaticValue,
) {
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let checks = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should exist");
    let options = checks
        .call
        .as_mut()
        .and_then(|call| call.first_arg.as_mut())
        .expect("checks options should exist");
    let G3TsAstroStaticValue::Object(properties) = options else {
        panic!("checks options should be an object");
    };

    if let Some(property) = properties.iter_mut().find(|property| property.key == key) {
        property.value = value;
    } else {
        properties.push(g3ts_astro_types::G3TsAstroStaticObjectProperty {
            key: key.to_owned(),
            value,
        });
    }
}

fn remove_nuasite_option(input: &mut G3TsAstroConfigChecksInput, key: &str) {
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let checks = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should exist");
    let options = checks
        .call
        .as_mut()
        .and_then(|call| call.first_arg.as_mut())
        .expect("checks options should exist");
    let G3TsAstroStaticValue::Object(properties) = options else {
        panic!("checks options should be an object");
    };

    properties.retain(|property| property.key != key);
}

fn duplicate_nuasite_option(
    input: &mut G3TsAstroConfigChecksInput,
    key: &str,
    value: G3TsAstroStaticValue,
) {
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let checks = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should exist");
    let options = checks
        .call
        .as_mut()
        .and_then(|call| call.first_arg.as_mut())
        .expect("checks options should exist");
    let G3TsAstroStaticValue::Object(properties) = options else {
        panic!("checks options should be an object");
    };

    properties.push(g3ts_astro_types::G3TsAstroStaticObjectProperty {
        key: key.to_owned(),
        value,
    });
}

fn replace_structured_data_check_source(
    input: &mut G3TsAstroConfigChecksInput,
    source_module: Option<String>,
) {
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let checks = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should exist");
    let options = checks
        .call
        .as_mut()
        .and_then(|call| call.first_arg.as_mut())
        .expect("checks options should exist");
    let G3TsAstroStaticValue::Object(properties) = options else {
        panic!("checks options should be an object");
    };
    let custom_checks = properties
        .iter_mut()
        .find(|property| property.key == "customChecks")
        .expect("customChecks should exist");
    let G3TsAstroStaticValue::Array(values) = &mut custom_checks.value else {
        panic!("customChecks should be an array");
    };
    values[0] = G3TsAstroStaticValue::ImportedIdentifier {
        local_name: "structuredDataPresentCheck".to_owned(),
        source_module,
        imported_name: Some("structuredDataPresentCheck".to_owned()),
    };
}

fn replace_structured_data_check_identity(
    input: &mut G3TsAstroConfigChecksInput,
    local_name: &str,
    imported_name: Option<String>,
) {
    let G3TsAstroConfigSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].astro_config
    else {
        panic!("golden astro config should be parsed");
    };
    let checks = snapshot
        .integrations
        .iter_mut()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should exist");
    let options = checks
        .call
        .as_mut()
        .and_then(|call| call.first_arg.as_mut())
        .expect("checks options should exist");
    let G3TsAstroStaticValue::Object(properties) = options else {
        panic!("checks options should be an object");
    };
    let custom_checks = properties
        .iter_mut()
        .find(|property| property.key == "customChecks")
        .expect("customChecks should exist");
    let G3TsAstroStaticValue::Array(values) = &mut custom_checks.value else {
        panic!("customChecks should be an array");
    };
    values[0] = G3TsAstroStaticValue::ImportedIdentifier {
        local_name: local_name.to_owned(),
        source_module: Some("g3ts-astro-nuasite-checks".to_owned()),
        imported_name,
    };
}
