use crate::BanDenyEntry;
use deny_toml_parser_assertions::parser as assertions;

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
targets = ["x86_64-unknown-linux-gnu"]
exclude = ["some-crate"]
"#,
    );

    let graph = cfg.graph.expect("graph should be present");
    assert_eq!(graph.all_features, Some(true), "all_features mismatch");
    assert_eq!(graph.no_default_features, Some(false), "no_default_features mismatch");
    assert_eq!(graph.targets.len(), 1, "should have 1 target");
    assert_eq!(graph.exclude, ["some-crate"], "exclude mismatch");
    assert!(graph.extra.is_empty(), "graph extra should be empty");
}

#[test]
fn advisories_section_parses() {
    let cfg = parse_fixture(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = [
    "RUSTSEC-2024-0001",
    { id = "RUSTSEC-2024-0002", reason = "Not applicable" },
]
"#,
    );

    let advisories = cfg.advisories.expect("advisories should be present");
    assert_eq!(advisories.unmaintained, Some("workspace".into()));
    assert_eq!(advisories.yanked, Some("warn".into()));
    assert_eq!(advisories.ignore.len(), 2, "should have 2 ignore entries");
    assert_eq!(
        advisories.ignore.first().map(crate::AdvisoryIgnoreEntry::id),
        Some("RUSTSEC-2024-0001"),
    );
    assert_eq!(
        advisories
            .ignore
            .get(1)
            .and_then(crate::AdvisoryIgnoreEntry::reason),
        Some("Not applicable"),
    );
}

#[test]
fn bans_section_parses_detailed_entries() {
    let cfg = parse_fixture(
        r#"
[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true

deny = [
    { name = "openssl", wrappers = [], reason = "Use rustls" },
    { crate = "regex", wrappers = ["tree-sitter"], reason = "Use structured parsers" },
]

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros"]
"#,
    );

    let bans = cfg.bans.expect("bans should be present");
    assert_eq!(bans.multiple_versions, Some("deny".into()));
    assert_eq!(bans.wildcards, Some("allow".into()));
    assert_eq!(bans.allow_wildcard_paths, Some(true));
    assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");
    assert_eq!(bans.features.len(), 1, "should have 1 feature entry");

    let first = bans.deny.first().and_then(|entry| match entry {
        BanDenyEntry::Detailed(detail) => Some(detail),
        BanDenyEntry::Simple(_) => None,
    });
    assert_eq!(first.and_then(|detail| detail.name()), Some("openssl"));
    assert_eq!(first.and_then(|detail| detail.reason()), Some("Use rustls"));

    let second = bans.deny.get(1).and_then(|entry| match entry {
        BanDenyEntry::Detailed(detail) => Some(detail),
        BanDenyEntry::Simple(_) => None,
    });
    assert_eq!(second.and_then(|detail| detail.crate_name()), Some("regex"));
    assert_eq!(
        second.map(|detail| detail.wrappers().iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["tree-sitter"]),
    );
}

#[test]
fn licenses_sources_and_output_sections_parse() {
    let cfg = parse_fixture(
        r#"
[licenses]
allow = ["MIT", "Apache-2.0"]
confidence-threshold = 0.8
exceptions = [{ name = "ring", allow = ["OpenSSL"] }]

[licenses.private]
ignore = true
registries = ["https://my-registry.example.com"]

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []

[output]
feature-depth = 1
"#,
    );

    let licenses = cfg.licenses.expect("licenses should be present");
    assert_eq!(licenses.allow, ["MIT", "Apache-2.0"]);
    assert_eq!(licenses.confidence_threshold, Some(0.8));
    assert_eq!(licenses.exceptions.len(), 1, "should have 1 exception");
    assert_eq!(
        licenses.exceptions.first().map(crate::LicenseException::name),
        Some("ring"),
    );
    assert_eq!(
        licenses
            .exceptions
            .first()
            .map(|entry| entry.allow().iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["OpenSSL"]),
    );

    let private = licenses.private.expect("private config should be present");
    assert_eq!(private.ignore(), Some(true));
    assert_eq!(private.registries(), ["https://my-registry.example.com"]);

    let sources = cfg.sources.expect("sources should be present");
    assert_eq!(sources.unknown_registry, Some("deny".into()));
    assert_eq!(sources.allow_registry, ["sparse+https://index.crates.io/"]);

    let output = cfg.output.expect("output should be present");
    assert_eq!(output.feature_depth, Some(1));
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

[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
skip = []
deny = [
    { name = "simd-json", wrappers = [], reason = "Ban competing JSON libraries" },
    { name = "openssl", wrappers = [], reason = "Ban OpenSSL (standardize on rustls)" },
]

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync"]

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
confidence-threshold = 0.8

[licenses.private]
ignore = true

[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );

    assert!(cfg.graph.is_some(), "graph should parse");
    assert!(cfg.bans.is_some(), "bans should parse");
    assert!(cfg.licenses.is_some(), "licenses should parse");
    assert!(cfg.advisories.is_some(), "advisories should parse");
    assert!(cfg.sources.is_some(), "sources should parse");
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
