#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use deny_toml_parser_runtime::types::{
    AdvisoryIgnoreEntry, AdvisoryScope, BanAllowEntry, BanBuildAllowBuildScriptEntry,
    BanDenyEntry, BanSkipEntry, BanSkipTreeEntry, DenyToml, GitSpec, GraphTargetEntry, Value,
};

pub fn assert_empty_toml(cfg: &DenyToml) {
    assert_eq!(cfg.graph, None, "graph should be None");
    assert_eq!(cfg.advisories, None, "advisories should be None");
    assert_eq!(cfg.bans, None, "bans should be None");
    assert_eq!(cfg.licenses, None, "licenses should be None");
    assert!(cfg.exceptions.is_empty(), "exceptions should be empty");
    assert_eq!(cfg.sources, None, "sources should be None");
    assert_eq!(cfg.output, None, "output should be None");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid deny.toml"),
        "expected error message prefix, got: {msg}",
    );
}

pub fn assert_graph_section(cfg: &DenyToml) {
    let graph = cfg.graph.as_ref().expect("graph should be present");
    assert_eq!(graph.all_features, Some(true), "all_features mismatch");
    assert_eq!(
        graph.no_default_features,
        Some(false),
        "no_default_features mismatch",
    );
    assert_eq!(graph.features, ["serde"]);
    assert_eq!(graph.exclude_dev, Some(true));
    assert_eq!(graph.exclude_unpublished, Some(true));
    assert_eq!(graph.targets.len(), 2, "should have 2 targets");
    assert_eq!(graph.exclude, ["some-crate"], "exclude mismatch");
    assert!(matches!(
        graph.targets.first(),
        Some(GraphTargetEntry::Simple(target)) if target == "x86_64-unknown-linux-gnu"
    ));
    let detailed = graph.targets.get(1).and_then(|target| match target {
        GraphTargetEntry::Detailed(detail) => Some(detail),
        GraphTargetEntry::Simple(_) => None,
    });
    assert_eq!(
        detailed.map(|detail| detail.triple.as_str()),
        Some("aarch64-apple-darwin")
    );
    assert_eq!(
        detailed.map(|detail| {
            detail
                .features
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        }),
        Some(vec!["neon"]),
    );
    assert_eq!(
        detailed.and_then(|detail| detail.extra.get("note")),
        Some(&Value::String("macos arm".into()))
    );
    assert!(graph.extra.is_empty(), "graph extra should be empty");
}

pub fn assert_advisories_section(cfg: &DenyToml) {
    let advisories = cfg.advisories.as_ref().expect("advisories should be present");
    assert_eq!(advisories.db_path, Some("~/.cargo/advisory-dbs".into()));
    assert_eq!(
        advisories.db_urls,
        ["https://github.com/RustSec/advisory-db"]
    );
    assert_eq!(advisories.version, Some(2));
    assert_eq!(advisories.unmaintained, Some(AdvisoryScope::Workspace));
    assert_eq!(advisories.unsound, Some(AdvisoryScope::Transitive));
    assert_eq!(advisories.yanked, Some("warn".into()));
    assert_eq!(advisories.git_fetch_with_cli, Some(true));
    assert_eq!(advisories.disable_yank_checking, Some(true));
    assert_eq!(advisories.maximum_db_staleness.as_deref(), Some("P30D"));
    assert_eq!(advisories.unused_ignored_advisory, Some("deny".into()));
    assert_eq!(advisories.ignore.len(), 3, "should have 3 ignore entries");
    assert!(matches!(
        advisories.ignore.first(),
        Some(AdvisoryIgnoreEntry::Simple(id)) if id == "RUSTSEC-2024-0001"
    ));
    let by_id = advisories.ignore.get(1).and_then(|entry| match entry {
        AdvisoryIgnoreEntry::Detailed(detail) => Some(detail),
        AdvisoryIgnoreEntry::Simple(_) => None,
    });
    assert_eq!(
        by_id.and_then(|detail| detail.id.as_deref()),
        Some("RUSTSEC-2024-0002")
    );
    assert_eq!(
        by_id.and_then(|detail| detail.reason.as_deref()),
        Some("Not applicable")
    );
    let by_crate = advisories.ignore.get(2).and_then(|entry| match entry {
        AdvisoryIgnoreEntry::Detailed(detail) => Some(detail),
        AdvisoryIgnoreEntry::Simple(_) => None,
    });
    assert_eq!(
        by_crate.and_then(|detail| detail.crate_name.as_deref()),
        Some("yanked@0.1.0")
    );
    assert_eq!(
        by_crate.and_then(|detail| detail.reason.as_deref()),
        Some("a new version has not been released"),
    );
    assert_eq!(
        by_crate.and_then(|detail| detail.extra.get("owner")),
        Some(&Value::String("security".into()))
    );
}

pub fn assert_detailed_bans(cfg: &DenyToml) {
    let bans = cfg.bans.as_ref().expect("bans should be present");
    assert_eq!(bans.multiple_versions, Some("deny".into()));
    assert_eq!(bans.multiple_versions_include_dev, Some(true));
    assert_eq!(bans.wildcards, Some("allow".into()));
    assert_eq!(bans.allow_wildcard_paths, Some(true));
    assert_eq!(bans.workspace_default_features, Some("warn".into()));
    assert_eq!(bans.external_default_features, Some("deny".into()));
    assert_eq!(bans.allow_workspace, Some(true));
    assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");

    let first = bans.deny.first().and_then(|entry| match entry {
        BanDenyEntry::Detailed(detail) => Some(detail),
        BanDenyEntry::Simple(_) => None,
    });
    assert_eq!(
        first.and_then(|detail| detail.name.as_deref()),
        Some("openssl")
    );
    assert_eq!(
        first.and_then(|detail| detail.reason.as_deref()),
        Some("Use rustls")
    );
    assert_eq!(
        first.and_then(|detail| detail.use_instead.as_deref()),
        Some("rustls")
    );

    let second = bans.deny.get(1).and_then(|entry| match entry {
        BanDenyEntry::Detailed(detail) => Some(detail),
        BanDenyEntry::Simple(_) => None,
    });
    assert_eq!(
        second.and_then(|detail| detail.crate_name.as_deref()),
        Some("regex")
    );
    assert_eq!(
        second.map(|detail| {
            detail
                .wrappers
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        }),
        Some(vec!["tree-sitter"]),
    );
    assert_eq!(
        second.and_then(|detail| detail.deny_multiple_versions),
        Some(true)
    );
}

pub fn assert_allow_and_skip_entries(cfg: &DenyToml) {
    let bans = cfg.bans.as_ref().expect("bans should be present");
    assert_eq!(bans.allow.len(), 1, "should have 1 allow entry");
    assert_eq!(bans.skip.len(), 1, "should have 1 skip entry");

    let allow = bans.allow.first().and_then(|entry| match entry {
        BanAllowEntry::Detailed(detail) => Some(detail),
        BanAllowEntry::Simple(_) => None,
    });
    assert_eq!(
        allow.and_then(|detail| detail.crate_name.as_deref()),
        Some("serde")
    );
    assert_eq!(
        allow.and_then(|detail| detail.version.as_deref()),
        Some("1.0")
    );
    assert_eq!(
        allow.and_then(|detail| detail.reason.as_deref()),
        Some("foundation crate")
    );
    assert_eq!(
        allow.and_then(|detail| detail.extra.get("source")),
        Some(&Value::String("workspace policy".into()))
    );

    let skip = bans.skip.first().and_then(|entry| match entry {
        BanSkipEntry::Detailed(detail) => Some(detail),
        BanSkipEntry::Simple(_) => None,
    });
    assert_eq!(
        skip.and_then(|detail| detail.crate_name.as_deref()),
        Some("windows-sys")
    );
    assert_eq!(
        skip.and_then(|detail| detail.version.as_deref()),
        Some("=0.48")
    );
    assert_eq!(
        skip.and_then(|detail| detail.reason.as_deref()),
        Some("temporary duplicate")
    );
    assert_eq!(
        skip.and_then(|detail| detail.extra.get("owner")),
        Some(&Value::String("platform".into()))
    );
}

pub fn assert_feature_entries(cfg: &DenyToml) {
    let bans = cfg.bans.as_ref().expect("bans should be present");
    let feature = bans
        .features
        .first()
        .expect("feature entry should be present");
    assert_eq!(feature.crate_name.as_deref(), Some("tokio"));
    assert_eq!(feature.version.as_deref(), Some("1.0"));
    assert_eq!(feature.deny, ["full"]);
    assert_eq!(feature.allow, ["rt-multi-thread", "macros"]);
    assert_eq!(
        feature.reason.as_deref(),
        Some("keep the runtime surface narrow")
    );
    assert_eq!(
        feature.extra.get("source"),
        Some(&Value::String("lint policy".into()))
    );
}

pub fn assert_skip_tree_workspace_and_build(cfg: &DenyToml) {
    let bans = cfg.bans.as_ref().expect("bans should be present");
    let skip_tree = bans.skip_tree.first().and_then(|entry| match entry {
        BanSkipTreeEntry::Detailed(detail) => Some(detail),
        BanSkipTreeEntry::Simple(_) => None,
    });
    assert_eq!(
        skip_tree.and_then(|detail| detail.crate_name.as_deref()),
        Some("windows-sys<=0.52")
    );
    assert_eq!(skip_tree.and_then(|detail| detail.depth), Some(3));
    assert_eq!(
        skip_tree.and_then(|detail| detail.reason.as_deref()),
        Some("legacy tree")
    );

    let workspace = bans
        .workspace_dependencies
        .as_ref()
        .expect("workspace-dependencies should be present");
    assert_eq!(workspace.duplicates, Some("allow".into()));
    assert_eq!(workspace.include_path_dependencies, Some(false));
    assert_eq!(workspace.unused, Some("warn".into()));

    let build = bans.build.as_ref().expect("build config should be present");
    let allow_build = build
        .allow_build_scripts
        .first()
        .and_then(|entry| match entry {
            BanBuildAllowBuildScriptEntry::Detailed(detail) => Some(detail),
            BanBuildAllowBuildScriptEntry::Simple(_) => None,
        });
    assert_eq!(
        allow_build.and_then(|detail| detail.crate_name.as_deref()),
        Some("cc@1.0")
    );
    assert_eq!(
        allow_build.and_then(|detail| detail.extra.get("note")),
        Some(&Value::String("approved builder".into()))
    );
    assert_eq!(build.executables, Some("warn".into()));
    assert_eq!(build.interpreted, Some("deny".into()));
    assert_eq!(build.script_extensions, ["cs"]);
    assert_eq!(build.enable_builtin_globs, Some(true));
    assert_eq!(build.include_dependencies, Some(true));
    assert_eq!(build.include_workspace, Some(true));
    assert_eq!(build.include_archives, Some(true));

    let bypass = build.bypass.first().expect("bypass should be present");
    assert_eq!(bypass.crate_name.as_deref(), Some("prost-build"));
    assert_eq!(
        bypass.build_script.as_deref(),
        Some("5392f0e58ad06e089462d93304dfe82337acbbefb87a0749a7dc2ed32af04af7")
    );
    assert_eq!(bypass.required_features, ["codegen"]);
    assert_eq!(bypass.allow_globs, ["scripts/*.cs"]);
    let allow_file = bypass.allow.first().expect("allow file should be present");
    assert_eq!(allow_file.path, "bin/x86_64-linux");
    assert_eq!(
        allow_file.checksum.as_deref(),
        Some("5392f0e58ad06e089462d93304dfe82337acbbefb87a0749a7dc2ed32af04af7")
    );
    assert_eq!(
        allow_file.extra.get("owner"),
        Some(&Value::String("build".into()))
    );
}

pub fn assert_top_level_exceptions(cfg: &DenyToml) {
    assert_eq!(cfg.exceptions.len(), 1, "should have 1 top-level exception");
    assert_eq!(
        cfg.exceptions
            .first()
            .and_then(|entry| entry.crate_name.as_deref()),
        Some("inferno")
    );
}

pub fn assert_licenses_allowlist_and_exceptions(cfg: &DenyToml) {
    let licenses = cfg.licenses.as_ref().expect("licenses should be present");
    assert_eq!(licenses.version, Some(2));
    assert_eq!(licenses.include_dev, Some(true));
    assert_eq!(licenses.include_build, Some(false));
    assert_eq!(licenses.unused_allowed_license, Some("deny".into()));
    assert_eq!(licenses.unused_license_exception, Some("warn".into()));
    assert_eq!(licenses.allow, ["MIT", "Apache-2.0"]);
    assert_eq!(licenses.confidence_threshold, Some(0.8));
    assert_eq!(licenses.exceptions.len(), 1, "should have 1 exception");
    assert_eq!(
        licenses
            .exceptions
            .first()
            .and_then(|entry| entry.crate_name.as_deref()),
        Some("ring"),
    );
    assert_eq!(
        licenses
            .exceptions
            .first()
            .and_then(|entry| entry.version.as_deref()),
        Some("0.17.8"),
    );
    assert_eq!(
        licenses
            .exceptions
            .first()
            .and_then(|entry| entry.reason.as_deref()),
        Some("upstream dual-license gap"),
    );
    assert_eq!(
        licenses.exceptions.first().map(|entry| {
            entry.allow.iter().map(String::as_str).collect::<Vec<_>>()
        }),
        Some(vec!["OpenSSL"]),
    );
    assert_eq!(
        licenses
            .exceptions
            .first()
            .and_then(|entry| entry.extra.get("tracker")),
        Some(&Value::String("SEC-123".into()))
    );
}

pub fn assert_licenses_clarify_and_private(cfg: &DenyToml) {
    let licenses = cfg.licenses.as_ref().expect("licenses should be present");
    let clarify = licenses
        .clarify
        .first()
        .expect("clarify entry should be present");
    assert_eq!(clarify.crate_name.as_deref(), Some("webpki"));
    assert_eq!(clarify.expression, "ISC");
    let license_file = clarify
        .license_files
        .first()
        .expect("license file should be present");
    assert_eq!(license_file.path, "LICENSE");
    assert_eq!(license_file.hash, 1_867_372);
    assert_eq!(
        license_file.extra.get("source"),
        Some(&Value::String("manual".into()))
    );

    let private = licenses.private.as_ref().expect("private config should be present");
    assert_eq!(private.ignore, Some(true));
    assert_eq!(private.registries, ["https://my-registry.example.com"]);
    assert_eq!(
        private.ignore_sources,
        ["https://sekretz.com/super/secret-index"]
    );
}

pub fn assert_sources_and_output(cfg: &DenyToml) {
    let sources = cfg.sources.as_ref().expect("sources should be present");
    assert_eq!(sources.unknown_registry, Some("deny".into()));
    assert_eq!(sources.unknown_git, Some("deny".into()));
    assert_eq!(sources.required_git_spec, Some(GitSpec::Tag));
    assert_eq!(sources.allow_registry, ["sparse+https://index.crates.io/"]);
    assert_eq!(
        sources.allow_git,
        ["https://github.com/EmbarkStudios/cargo-deny"]
    );
    assert_eq!(sources.private, ["https://internal-host/repos"]);
    assert_eq!(sources.unused_allowed_source, Some("warn".into()));
    let allow_org = sources.allow_org.as_ref().expect("allow-org should be present");
    assert_eq!(allow_org.github, ["YourOrg"]);
    assert_eq!(allow_org.gitlab, ["gitlab-org"]);
    assert_eq!(allow_org.bitbucket, ["atlassian"]);

    let output = cfg.output.as_ref().expect("output should be present");
    assert_eq!(output.feature_depth, Some(1));
}

pub fn assert_unknown_keys_land_in_extra(cfg: &DenyToml) {
    let graph = cfg.graph.as_ref().expect("graph should be present");
    assert_eq!(graph.extra.len(), 1, "graph should have 1 extra key");
    assert!(cfg.extra.contains_key("some-future-section"));
}

pub fn assert_representative_config(cfg: &DenyToml) {
    assert!(cfg.graph.is_some(), "graph should parse");
    assert!(cfg.bans.is_some(), "bans should parse");
    assert!(cfg.licenses.is_some(), "licenses should parse");
    assert!(cfg.advisories.is_some(), "advisories should parse");
    assert!(cfg.sources.is_some(), "sources should parse");
    assert!(cfg.exceptions.is_empty(), "root exceptions should be empty");
    assert!(cfg.extra.is_empty(), "all top-level keys should be known");
}

pub fn assert_graph_all_features(cfg: &DenyToml) {
    let graph = cfg.graph.as_ref().expect("graph should be present");
    assert_eq!(graph.all_features, Some(true), "all_features mismatch");
}
