use crate::{AdvisoryIgnoreEntry, AdvisoryScope, BanDenyEntry, GitSpec, GraphTargetEntry};
use deny_toml_parser_runtime_assertions::parser as assertions;

use super::helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn empty_string_yields_empty_toml() {
    let cfg = parse_fixture("");
    assertions::assert_empty_toml(&cfg);
}

#[test]
fn graph_section_parses() {
    let cfg = parse_fixture(
        r#"
[graph]
all-features = true
no-default-features = false
features = ["serde"]
exclude-dev = true
exclude-unpublished = true
targets = [
    "x86_64-unknown-linux-gnu",
    { triple = "aarch64-apple-darwin", features = ["neon"], note = "macos arm" },
]
exclude = ["some-crate"]
"#,
    );

    let graph = cfg.graph.expect("graph should be present");
    assert_eq!(graph.all_features, Some(true), "all_features mismatch");
    assert_eq!(graph.no_default_features, Some(false), "no_default_features mismatch");
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
    assert_eq!(detailed.map(|detail| detail.triple.as_str()), Some("aarch64-apple-darwin"));
    assert_eq!(
        detailed.map(|detail| detail.features.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["neon"]),
    );
    assert_eq!(
        detailed.and_then(|detail| detail.extra.get("note")),
        Some(&toml::Value::String("macos arm".into()))
    );
    assert!(graph.extra.is_empty(), "graph extra should be empty");
}

#[test]
fn advisories_section_parses() {
    let cfg = parse_fixture(
        r#"
[advisories]
db-path = "~/.cargo/advisory-dbs"
db-urls = ["https://github.com/RustSec/advisory-db"]
version = 2
unmaintained = "workspace"
unsound = "transitive"
yanked = "warn"
git-fetch-with-cli = true
disable-yank-checking = true
maximum-db-staleness = "P30D"
unused-ignored-advisory = "deny"
ignore = [
    "RUSTSEC-2024-0001",
    { id = "RUSTSEC-2024-0002", reason = "Not applicable" },
    { crate = "yanked@0.1.0", reason = "a new version has not been released", owner = "security" },
]
"#,
    );

    let advisories = cfg.advisories.expect("advisories should be present");
    assert_eq!(advisories.db_path, Some("~/.cargo/advisory-dbs".into()));
    assert_eq!(advisories.db_urls, ["https://github.com/RustSec/advisory-db"]);
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
    assert_eq!(by_id.and_then(|detail| detail.id.as_deref()), Some("RUSTSEC-2024-0002"));
    assert_eq!(by_id.and_then(|detail| detail.reason.as_deref()), Some("Not applicable"));
    let by_crate = advisories.ignore.get(2).and_then(|entry| match entry {
        AdvisoryIgnoreEntry::Detailed(detail) => Some(detail),
        AdvisoryIgnoreEntry::Simple(_) => None,
    });
    assert_eq!(by_crate.and_then(|detail| detail.crate_name.as_deref()), Some("yanked@0.1.0"));
    assert_eq!(
        by_crate.and_then(|detail| detail.reason.as_deref()),
        Some("a new version has not been released"),
    );
    assert_eq!(
        by_crate.and_then(|detail| detail.extra.get("owner")),
        Some(&toml::Value::String("security".into()))
    );
}

#[test]
fn bans_section_parses_detailed_deny_entries() {
    let cfg = parse_fixture(
        r#"
[bans]
multiple-versions = "deny"
multiple-versions-include-dev = true
wildcards = "allow"
allow-wildcard-paths = true
workspace-default-features = "warn"
external-default-features = "deny"
allow-workspace = true

deny = [
    { name = "openssl", wrappers = [], reason = "Use rustls", use-instead = "rustls" },
    { crate = "regex", wrappers = ["tree-sitter"], reason = "Use structured parsers", deny-multiple-versions = true },
]
"#,
    );

    let bans = cfg.bans.expect("bans should be present");
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
        second.map(|detail| detail.wrappers.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["tree-sitter"]),
    );
    assert_eq!(
        second.and_then(|detail| detail.deny_multiple_versions),
        Some(true)
    );
}

#[test]
fn bans_section_parses_detailed_allow_and_skip_entries() {
    let cfg = parse_fixture(
        r#"
[bans]
allow = [
    { crate = "serde", version = "1.0", reason = "foundation crate", source = "workspace policy" },
]

skip = [
    { crate = "windows-sys", version = "=0.48", reason = "temporary duplicate", owner = "platform" },
]
"#,
    );

    let bans = cfg.bans.expect("bans should be present");
    assert_eq!(bans.allow.len(), 1, "should have 1 allow entry");
    assert_eq!(bans.skip.len(), 1, "should have 1 skip entry");

    let allow = bans.allow.first().and_then(|entry| match entry {
        crate::BanAllowEntry::Detailed(detail) => Some(detail),
        crate::BanAllowEntry::Simple(_) => None,
    });
    assert_eq!(
        allow.and_then(|detail| detail.crate_name.as_deref()),
        Some("serde")
    );
    assert_eq!(allow.and_then(|detail| detail.version.as_deref()), Some("1.0"));
    assert_eq!(
        allow.and_then(|detail| detail.reason.as_deref()),
        Some("foundation crate")
    );
    assert_eq!(
        allow.and_then(|detail| detail.extra.get("source")),
        Some(&toml::Value::String("workspace policy".into()))
    );

    let skip = bans.skip.first().and_then(|entry| match entry {
        crate::BanSkipEntry::Detailed(detail) => Some(detail),
        crate::BanSkipEntry::Simple(_) => None,
    });
    assert_eq!(
        skip.and_then(|detail| detail.crate_name.as_deref()),
        Some("windows-sys")
    );
    assert_eq!(skip.and_then(|detail| detail.version.as_deref()), Some("=0.48"));
    assert_eq!(
        skip.and_then(|detail| detail.reason.as_deref()),
        Some("temporary duplicate")
    );
    assert_eq!(
        skip.and_then(|detail| detail.extra.get("owner")),
        Some(&toml::Value::String("platform".into()))
    );
}

#[test]
fn bans_section_parses_feature_entries_with_crate_and_reason() {
    let cfg = parse_fixture(
        r#"
[bans]

[[bans.features]]
crate = "tokio"
version = "1.0"
deny = ["full"]
allow = ["rt-multi-thread", "macros"]
reason = "keep the runtime surface narrow"
source = "lint policy"
"#,
    );

    let bans = cfg.bans.expect("bans should be present");
    let feature = bans.features.first().expect("feature entry should be present");
    assert_eq!(feature.crate_name.as_deref(), Some("tokio"));
    assert_eq!(feature.version.as_deref(), Some("1.0"));
    assert_eq!(feature.deny, ["full"]);
    assert_eq!(feature.allow, ["rt-multi-thread", "macros"]);
    assert_eq!(feature.reason.as_deref(), Some("keep the runtime surface narrow"));
    assert_eq!(
        feature.extra.get("source"),
        Some(&toml::Value::String("lint policy".into()))
    );
}

#[test]
fn bans_section_parses_skip_tree_workspace_dependencies_and_build() {
    let cfg = parse_fixture(
        r#"
[bans]
skip-tree = [
    { crate = "windows-sys<=0.52", depth = 3, reason = "legacy tree" },
]

[bans.workspace-dependencies]
duplicates = "allow"
include-path-dependencies = false
unused = "warn"

[bans.build]
allow-build-scripts = [{ crate = "cc@1.0", note = "approved builder" }]
executables = "warn"
interpreted = "deny"
script-extensions = ["cs"]
enable-builtin-globs = true
include-dependencies = true
include-workspace = true
include-archives = true

[[bans.build.bypass]]
crate = "prost-build"
build-script = "5392f0e58ad06e089462d93304dfe82337acbbefb87a0749a7dc2ed32af04af7"
required-features = ["codegen"]
allow-globs = ["scripts/*.cs"]
allow = [
  { path = "bin/x86_64-linux", checksum = "5392f0e58ad06e089462d93304dfe82337acbbefb87a0749a7dc2ed32af04af7", owner = "build" },
]
"#,
    );

    let bans = cfg.bans.expect("bans should be present");
    let skip_tree = bans.skip_tree.first().and_then(|entry| match entry {
        crate::BanSkipTreeEntry::Detailed(detail) => Some(detail),
        crate::BanSkipTreeEntry::Simple(_) => None,
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
        .expect("workspace-dependencies should be present");
    assert_eq!(workspace.duplicates, Some("allow".into()));
    assert_eq!(workspace.include_path_dependencies, Some(false));
    assert_eq!(workspace.unused, Some("warn".into()));

    let build = bans.build.expect("build config should be present");
    let allow_build = build
        .allow_build_scripts
        .first()
        .and_then(|entry| match entry {
            crate::BanBuildAllowBuildScriptEntry::Detailed(detail) => Some(detail),
            crate::BanBuildAllowBuildScriptEntry::Simple(_) => None,
        });
    assert_eq!(
        allow_build.and_then(|detail| detail.crate_name.as_deref()),
        Some("cc@1.0")
    );
    assert_eq!(
        allow_build.and_then(|detail| detail.extra.get("note")),
        Some(&toml::Value::String("approved builder".into()))
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
        Some(&toml::Value::String("build".into()))
    );
}

#[test]
fn top_level_exceptions_parse() {
    let cfg = parse_fixture(
        r#"
exceptions = [
  { crate = "inferno", allow = ["CDDL-1.0"], reason = "repo-local exception" },
]
"#,
    );

    assert_eq!(cfg.exceptions.len(), 1, "should have 1 top-level exception");
    assert_eq!(
        cfg.exceptions.first().and_then(|entry| entry.crate_name.as_deref()),
        Some("inferno")
    );
}

#[test]
fn licenses_parse_allowlist_and_exceptions() {
    let cfg = parse_fixture(
        r#"

[licenses]
version = 2
include-dev = true
include-build = false
unused-allowed-license = "deny"
unused-license-exception = "warn"
allow = ["MIT", "Apache-2.0"]
confidence-threshold = 0.8
exceptions = [
    { crate = "ring", version = "0.17.8", allow = ["OpenSSL"], reason = "upstream dual-license gap", tracker = "SEC-123" }
]
"#,
    );

    let licenses = cfg.licenses.expect("licenses should be present");
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
        licenses
            .exceptions
            .first()
            .map(|entry| entry.allow.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["OpenSSL"]),
    );
    assert_eq!(
        licenses
            .exceptions
            .first()
            .and_then(|entry| entry.extra.get("tracker")),
        Some(&toml::Value::String("SEC-123".into()))
    );
}

#[test]
fn licenses_parse_clarify_and_private() {
    let cfg = parse_fixture(
        r#"
[licenses]
clarify = [
    { crate = "webpki", expression = "ISC", license-files = [{ path = "LICENSE", hash = 0x001c7e6c, source = "manual" }] }
]

[licenses.private]
ignore = true
registries = ["https://my-registry.example.com"]
ignore-sources = ["https://sekretz.com/super/secret-index"]
"#,
    );

    let licenses = cfg.licenses.expect("licenses should be present");
    let clarify = licenses.clarify.first().expect("clarify entry should be present");
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
        Some(&toml::Value::String("manual".into()))
    );

    let private = licenses.private.expect("private config should be present");
    assert_eq!(private.ignore, Some(true));
    assert_eq!(private.registries, ["https://my-registry.example.com"]);
    assert_eq!(private.ignore_sources, ["https://sekretz.com/super/secret-index"]);
}

#[test]
fn sources_and_output_parse() {
    let cfg = parse_fixture(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
required-git-spec = "tag"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = ["https://github.com/EmbarkStudios/cargo-deny"]
private = ["https://internal-host/repos"]
unused-allowed-source = "warn"

[sources.allow-org]
github = ["YourOrg"]
gitlab = ["gitlab-org"]
bitbucket = ["atlassian"]

[output]
feature-depth = 1
"#,
    );

    let sources = cfg.sources.expect("sources should be present");
    assert_eq!(sources.unknown_registry, Some("deny".into()));
    assert_eq!(sources.unknown_git, Some("deny".into()));
    assert_eq!(sources.required_git_spec, Some(GitSpec::Tag));
    assert_eq!(sources.allow_registry, ["sparse+https://index.crates.io/"]);
    assert_eq!(sources.allow_git, ["https://github.com/EmbarkStudios/cargo-deny"]);
    assert_eq!(sources.private, ["https://internal-host/repos"]);
    assert_eq!(sources.unused_allowed_source, Some("warn".into()));
    let allow_org = sources.allow_org.expect("allow-org should be present");
    assert_eq!(allow_org.github, ["YourOrg"]);
    assert_eq!(allow_org.gitlab, ["gitlab-org"]);
    assert_eq!(allow_org.bitbucket, ["atlassian"]);

    let output = cfg.output.expect("output should be present");
    assert_eq!(output.feature_depth, Some(1));
}

#[test]
fn invalid_required_git_spec_is_rejected() {
    let err = crate::parse(
        r#"
[sources]
required-git-spec = "commit"
"#,
    )
    .expect_err("invalid git spec should fail");

    assertions::assert_parse_error(err);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
[graph]
all-features = true
some-new-graph-option = "test"

[some-future-section]
key = "value"
"#,
    );

    let graph = cfg.graph.expect("graph should be present");
    assert_eq!(graph.extra.len(), 1, "graph should have 1 extra key");
    assert!(cfg.extra.contains_key("some-future-section"));
}

#[test]
fn representative_config_parses() {
    let cfg = parse_fixture(
        r#"
[graph]
all-features = true
no-default-features = false
features = ["some-feature"]
exclude-dev = true
exclude-unpublished = true

[bans]
multiple-versions = "deny"
multiple-versions-include-dev = false
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
workspace-default-features = "warn"
external-default-features = "deny"
allow-workspace = false
skip = []
skip-tree = []
deny = [
    { name = "simd-json", wrappers = [], reason = "Ban competing JSON libraries", use-instead = "serde_json" },
    { name = "openssl", wrappers = [], reason = "Ban OpenSSL (standardize on rustls)" },
]

[[bans.features]]
name = "tokio"
version = "1.0"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync"]

[licenses]
include-build = true
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
confidence-threshold = 0.8

[licenses.private]
ignore = true

[advisories]
db-urls = ["https://github.com/RustSec/advisory-db"]
unmaintained = "workspace"
yanked = "warn"
ignore = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
required-git-spec = "rev"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );

    assert!(cfg.graph.is_some(), "graph should parse");
    assert!(cfg.bans.is_some(), "bans should parse");
    assert!(cfg.licenses.is_some(), "licenses should parse");
    assert!(cfg.advisories.is_some(), "advisories should parse");
    assert!(cfg.sources.is_some(), "sources should parse");
    assert!(cfg.exceptions.is_empty(), "root exceptions should be empty");
    assert!(cfg.extra.is_empty(), "all top-level keys should be known");
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r"
[graph]
all-features = true
",
    );

    let graph = cfg.graph.expect("graph should be present");
    assert_eq!(graph.all_features, Some(true), "all_features mismatch");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
