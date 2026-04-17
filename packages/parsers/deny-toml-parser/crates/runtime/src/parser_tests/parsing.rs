use deny_toml_parser_runtime_assertions::parser as assertions;

use super::helpers::{parse_error, parse_fixture, parse_from_tempfile};

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

    assertions::assert_graph_section(&cfg);
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

    assertions::assert_advisories_section(&cfg);
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

    assertions::assert_detailed_bans(&cfg);
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

    assertions::assert_allow_and_skip_entries(&cfg);
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

    assertions::assert_feature_entries(&cfg);
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

    assertions::assert_skip_tree_workspace_and_build(&cfg);
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

    assertions::assert_top_level_exceptions(&cfg);
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

    assertions::assert_licenses_allowlist_and_exceptions(&cfg);
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

    assertions::assert_licenses_clarify_and_private(&cfg);
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

    assertions::assert_sources_and_output(&cfg);
}

#[test]
fn invalid_required_git_spec_is_rejected() {
    let err = parse_error(
        r#"
[sources]
required-git-spec = "commit"
"#,
    );

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

    assertions::assert_unknown_keys_land_in_extra(&cfg);
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

    assertions::assert_representative_config(&cfg);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r"
[graph]
all-features = true
",
    );

    assertions::assert_graph_all_features(&cfg);
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = parse_error("this is not [[[valid toml");
    assertions::assert_parse_error(err);
}
