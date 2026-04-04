use crate::config::DenyConfig;
use crate::bans::BanDenyEntry;

/// Parse helper that panics with context on failure.
fn parse(input: &str) -> DenyConfig {
    input.parse::<DenyConfig>().expect("should parse valid deny.toml")
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse("");

    assert_eq!(cfg.graph, None, "graph should be None");
    assert_eq!(cfg.advisories, None, "advisories should be None");
    assert_eq!(cfg.bans, None, "bans should be None");
    assert_eq!(cfg.licenses, None, "licenses should be None");
    assert_eq!(cfg.sources, None, "sources should be None");
    assert_eq!(cfg.output, None, "output should be None");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

#[test]
fn graph_section_parses() {
    let cfg = parse(
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
    assert_eq!(graph.exclude.len(), 1, "should have 1 exclude");
    assert_eq!(graph.exclude[0], "some-crate", "exclude value mismatch");
    assert!(graph.extra.is_empty(), "graph extra should be empty");
}

#[test]
fn advisories_section_parses() {
    let cfg = parse(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []
"#,
    );

    let adv = cfg.advisories.expect("advisories should be present");
    assert_eq!(adv.unmaintained, Some("workspace".into()), "unmaintained mismatch");
    assert_eq!(adv.yanked, Some("warn".into()), "yanked mismatch");
    assert!(adv.ignore.is_empty(), "ignore should be empty");
    assert!(adv.extra.is_empty(), "advisories extra should be empty");
}

#[test]
fn advisories_deprecated_fields_parse() {
    let cfg = parse(
        r#"
[advisories]
vulnerability = "deny"
notice = "warn"
unsound = "deny"
"#,
    );

    let adv = cfg.advisories.expect("advisories should be present");
    assert_eq!(adv.vulnerability, Some("deny".into()), "vulnerability mismatch");
    assert_eq!(adv.notice, Some("warn".into()), "notice mismatch");
    assert_eq!(adv.unsound, Some("deny".into()), "unsound mismatch");
}

#[test]
fn advisory_ignore_entries() {
    let cfg = parse(
        r#"
[advisories]
ignore = [
    "RUSTSEC-2024-0001",
    { id = "RUSTSEC-2024-0002", reason = "Not applicable" },
]
"#,
    );

    let adv = cfg.advisories.expect("advisories should be present");
    assert_eq!(adv.ignore.len(), 2, "should have 2 ignore entries");
    assert_eq!(adv.ignore[0].id(), "RUSTSEC-2024-0001", "first ignore ID");
    assert_eq!(adv.ignore[0].reason(), None, "simple entry has no reason");
    assert_eq!(adv.ignore[1].id(), "RUSTSEC-2024-0002", "second ignore ID");
    assert_eq!(
        adv.ignore[1].reason(),
        Some("Not applicable"),
        "detailed entry should have reason",
    );
}

#[test]
fn bans_simple_deny_entries() {
    let cfg = parse(
        r#"
[bans]
multiple-versions = "deny"
deny = ["openssl", "chrono"]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.multiple_versions, Some("deny".into()), "multiple_versions mismatch");
    assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");
    assert_eq!(bans.deny[0].name(), Some("openssl"), "first deny name");
    assert_eq!(bans.deny[1].name(), Some("chrono"), "second deny name");
}

#[test]
fn bans_detailed_deny_entries() {
    let cfg = parse(
        r#"
[bans]
deny = [
    { name = "openssl", wrappers = [], reason = "Use rustls" },
    { name = "regex", wrappers = ["tree-sitter", "globset"], reason = "Use structured parsers" },
]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");

    let first = match &bans.deny[0] {
        BanDenyEntry::Detailed(d) => d,
        BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
    };
    assert_eq!(first.name(), Some("openssl"), "first entry name");
    assert_eq!(first.reason(), Some("Use rustls"), "first entry reason");
    assert!(first.wrappers().is_empty(), "openssl should have no wrappers");

    let second = match &bans.deny[1] {
        BanDenyEntry::Detailed(d) => d,
        BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
    };
    assert_eq!(second.wrappers().len(), 2, "regex should have 2 wrappers");
    assert_eq!(second.wrappers()[0], "tree-sitter", "first wrapper");
    assert_eq!(second.wrappers()[1], "globset", "second wrapper");
}

#[test]
fn bans_skip_entries() {
    let cfg = parse(
        r#"
[bans]
skip = [
    "windows-sys",
    { name = "syn", version = "=1", reason = "Transitive via proc-macro2" },
]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.skip.len(), 2, "should have 2 skip entries");
    assert_eq!(bans.skip[0].name(), Some("windows-sys"), "first skip name");
    assert_eq!(bans.skip[0].reason(), None, "simple skip has no reason");
    assert_eq!(bans.skip[1].name(), Some("syn"), "second skip name");
    assert_eq!(
        bans.skip[1].reason(),
        Some("Transitive via proc-macro2"),
        "detailed skip reason",
    );
}

#[test]
fn bans_allow_entries() {
    let cfg = parse(
        r#"
[bans]
allow = [
    "serde",
    { name = "tokio", version = "1" },
]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.allow.len(), 2, "should have 2 allow entries");
    assert_eq!(bans.allow[0].name(), Some("serde"), "first allow name");
    assert_eq!(bans.allow[1].name(), Some("tokio"), "second allow name");
}

#[test]
fn bans_feature_entries() {
    let cfg = parse(
        r#"
[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync"]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.features.len(), 1, "should have 1 feature entry");
    let feat = &bans.features[0];
    assert_eq!(feat.name(), Some("tokio"), "feature entry name");
    assert_eq!(feat.deny(), &["full"], "denied features");
    assert_eq!(feat.allow().len(), 4, "should have 4 allowed features");
}

#[test]
fn bans_wildcard_settings() {
    let cfg = parse(
        r#"
[bans]
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.wildcards, Some("allow".into()), "wildcards mismatch");
    assert_eq!(bans.allow_wildcard_paths, Some(true), "allow_wildcard_paths mismatch");
    assert_eq!(bans.highlight, Some("all".into()), "highlight mismatch");
}

#[test]
fn licenses_section_parses() {
    let cfg = parse(
        r#"
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
confidence-threshold = 0.8

[licenses.private]
ignore = true
"#,
    );

    let lic = cfg.licenses.expect("deny config should have [licenses] section");
    assert_eq!(lic.allow.len(), 3, "should have 3 allowed licenses");
    assert_eq!(lic.allow[0], "MIT", "first license");
    assert_eq!(lic.confidence_threshold, Some(0.8), "confidence threshold mismatch");

    let private = lic.private.expect("private should be present");
    assert_eq!(private.ignore(), Some(true), "private.ignore mismatch");
    assert!(private.registries().is_empty(), "private.registries should be empty");
}

#[test]
fn license_exceptions_parse() {
    let cfg = parse(
        r#"
[licenses]
allow = ["MIT"]
exceptions = [
    { name = "ring", allow = ["OpenSSL"] },
    { name = "unicode-ident", allow = ["Unicode-DFS-2016"] },
]
"#,
    );

    let lic = cfg.licenses.expect("deny config should have [licenses] section");
    assert_eq!(lic.exceptions.len(), 2, "should have 2 exceptions");
    assert_eq!(lic.exceptions[0].name(), "ring", "first exception name");
    assert_eq!(lic.exceptions[0].allow(), &["OpenSSL"], "first exception allow");
    assert_eq!(lic.exceptions[1].name(), "unicode-ident", "second exception name");
}

#[test]
fn sources_section_parses() {
    let cfg = parse(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );

    let src = cfg.sources.expect("sources should be present");
    assert_eq!(src.unknown_registry, Some("deny".into()), "unknown_registry mismatch");
    assert_eq!(src.unknown_git, Some("deny".into()), "unknown_git mismatch");
    assert_eq!(src.allow_registry.len(), 1, "should have 1 allowed registry");
    assert_eq!(
        src.allow_registry[0], "sparse+https://index.crates.io/",
        "registry URL mismatch",
    );
    assert!(src.allow_git.is_empty(), "allow_git should be empty");
    assert!(src.extra.is_empty(), "sources extra should be empty");
}

#[test]
fn output_section_parses() {
    let cfg = parse(
        r#"
[output]
feature-depth = 1
"#,
    );

    let out = cfg.output.expect("output should be present");
    assert_eq!(out.feature_depth, Some(1), "feature_depth mismatch");
    assert!(out.extra.is_empty(), "output extra should be empty");
}

#[test]
fn unknown_top_level_keys_land_in_extra() {
    let cfg = parse(
        r#"
[graph]
all-features = true

[some-future-section]
key = "value"
"#,
    );

    assert!(cfg.graph.is_some(), "graph should parse");
    assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown top-level key");
    assert!(
        cfg.extra.contains_key("some-future-section"),
        "unknown section should be captured",
    );
}

#[test]
fn unknown_keys_in_nested_sections() {
    let cfg = parse(
        r#"
[graph]
all-features = true
some-new-graph-option = "test"

[advisories]
unmaintained = "deny"
new-advisory-field = 42

[bans]
multiple-versions = "deny"
new-ban-option = true

[licenses]
allow = ["MIT"]
new-license-field = "test"

[sources]
unknown-registry = "deny"
new-source-field = false

[output]
feature-depth = 1
new-output-field = "hello"
"#,
    );

    let graph = cfg.graph.expect("graph should be present");
    assert_eq!(graph.extra.len(), 1, "graph should have 1 extra key");

    let adv = cfg.advisories.expect("advisories should be present");
    assert_eq!(adv.extra.len(), 1, "advisories should have 1 extra key");

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.extra.len(), 1, "bans should have 1 extra key");

    let lic = cfg.licenses.expect("deny config should have [licenses] section");
    assert_eq!(lic.extra.len(), 1, "licenses should have 1 extra key");

    let src = cfg.sources.expect("sources should be present");
    assert_eq!(src.extra.len(), 1, "sources should have 1 extra key");

    let out = cfg.output.expect("output should be present");
    assert_eq!(out.extra.len(), 1, "output should have 1 extra key");
}

#[test]
fn real_deny_toml_parses() {
    let cfg = parse(
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
    { name = "regex", wrappers = ["tree-sitter", "globset", "ignore"], reason = "Ban regex crates" },
]

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Unicode-3.0",
    "Zlib",
    "CC0-1.0",
    "OpenSSL",
    "BSL-1.0",
    "MPL-2.0",
]
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

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.deny.len(), 3, "should have 3 deny entries");
    assert_eq!(bans.features.len(), 1, "should have 1 feature entry");

    let lic = cfg.licenses.expect("deny config should have [licenses] section");
    assert_eq!(lic.allow.len(), 12, "should have 12 allowed licenses");
    assert!(lic.private.is_some(), "private config should be present");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = bad.parse::<DenyConfig>();
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assert!(
        msg.contains("invalid deny.toml"),
        "expected error message prefix, got: {msg}",
    );
}

#[test]
fn ban_deny_crate_field_parses() {
    let cfg = parse(
        r#"
[bans]
deny = [
    { crate = "openssl", reason = "Use rustls" },
]
"#,
    );

    let bans = cfg.bans.expect("deny config should have [bans] section");
    assert_eq!(bans.deny.len(), 1, "should have 1 deny entry");

    let entry = match &bans.deny[0] {
        BanDenyEntry::Detailed(d) => d,
        BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
    };
    assert_eq!(entry.crate_name(), Some("openssl"), "crate field mismatch");
    assert_eq!(entry.name(), None, "name should be None when using crate field");
}

#[test]
fn license_private_with_registries() {
    let cfg = parse(
        r#"
[licenses]
allow = ["MIT"]

[licenses.private]
ignore = true
registries = ["https://my-registry.example.com"]
"#,
    );

    let lic = cfg.licenses.expect("deny config should have [licenses] section");
    let private = lic.private.expect("private should be present");
    assert_eq!(private.ignore(), Some(true), "private.ignore mismatch");
    assert_eq!(private.registries().len(), 1, "should have 1 registry");
    assert_eq!(
        private.registries()[0], "https://my-registry.example.com",
        "registry URL mismatch",
    );
}
