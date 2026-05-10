use g3ts_jscpd_ingestion_assertions::run as assertions;

const VALID_ROOT: &str =
    r#"{ "threshold": 0, "minTokens": 50, "absolute": true, "format": ["typescript"] }"#;
const VALID_ALT_ROOT: &str =
    r#"{ "threshold": 1, "minTokens": 50, "absolute": false, "format": ["typescript"] }"#;
const NESTED_PACKAGE_JSON: &str = r#"{ "name": "landing" }"#;

#[test]
fn returns_missing_when_root_jscpd_is_absent() {
    let (_dir, input) = super::helpers::ingest_with_files(&[], "");
    assertions::assert_root_missing(&input);
}

#[test]
fn parses_root_jscpd_when_present() {
    let (_dir, input) = super::helpers::ingest_with_files(&[(".jscpd.json", VALID_ROOT)], "");
    assertions::assert_root_parsed(&input, ".jscpd.json");
}

#[test]
fn parses_ancestor_root_jscpd_when_validating_nested_app_root() {
    let seed: [super::helpers::SeedFile<'_>; 2] = [
        (".jscpd.json", VALID_ROOT),
        ("apps/landing/package.json", NESTED_PACKAGE_JSON),
    ];
    let (_dir, input) = super::helpers::ingest_with_files(&seed, "apps/landing");
    assertions::assert_root_parsed(&input, "../../.jscpd.json");
}

#[test]
fn prefers_nested_root_jscpd_over_ancestor_root_jscpd() {
    let outer: super::helpers::SeedFile<'_> = (".jscpd.json", VALID_ALT_ROOT);
    let inner: super::helpers::SeedFile<'_> = ("apps/landing/.jscpd.json", VALID_ROOT);
    let (_dir, input) = super::helpers::ingest_with_files(&[outer, inner], "apps/landing");
    assertions::assert_root_parsed(&input, ".jscpd.json");
}

#[test]
fn surfaces_parse_error_for_invalid_root_jscpd() {
    let (_dir, input) = super::helpers::ingest_with_files(&[(".jscpd.json", "{ invalid ")], "");
    assertions::assert_root_parse_error(&input, ".jscpd.json");
}

#[test]
fn surfaces_parse_error_for_invalid_ancestor_root_jscpd() {
    let (_dir, input) = super::helpers::ingest_with_files(
        &[
            (".jscpd.json", "{ invalid "),
            ("apps/landing/package.json", NESTED_PACKAGE_JSON),
        ],
        "apps/landing",
    );
    assertions::assert_root_parse_error(&input, "../../.jscpd.json");
}

#[cfg(unix)]
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Unix-only fixture flips permissions via std::fs::set_permissions; routing through \
              the production fs port would require the test sidecar to call a sibling module, \
              which is forbidden by the runtime-assertions-split rule"
)]
fn surfaces_unreadable_for_unreadable_root_jscpd() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let (dir, _initial_input) =
        super::helpers::ingest_with_files(&[(".jscpd.json", r#"{ "threshold": 0 }"#)], "");

    let unreadable_permissions = fs::Permissions::from_mode(0o000);
    fs::set_permissions(dir.path().join(".jscpd.json"), unreadable_permissions)
        .expect("chmod 000 should succeed on fixture file");

    let crawl = g3_workspace_crawl::crawl(dir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    assertions::assert_root_unreadable(&input, ".jscpd.json");
}
