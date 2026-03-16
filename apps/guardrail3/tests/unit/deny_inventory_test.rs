use std::path::Path;

use guardrail3::app::rs::validate::deny_inventory::*;

// ---- Bug 6: deny.toml skip entry parsing ----

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
#[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
fn skip_entry_parses_crate_at_version_format() {
    let deny_content = r#"
[bans]
skip = [
    { crate = "windows-sys@0.60.2", reason = "transitive dep conflict" },
]
"#;
    let table: toml::Value = deny_content.parse().expect("valid TOML");
    let mut results = Vec::new();
    let path = Path::new("deny.toml");
    check_skip_entries(&table, path, &mut results);
    assert!(
        !results.is_empty(),
        "Should produce a result for skip entry"
    );
    assert!(
        results[0].message.contains("windows-sys"),
        "Should parse crate name from crate@version format, got: {}",
        results[0].message
    );
    assert!(
        results[0].message.contains("0.60.2"),
        "Should parse version from crate@version format, got: {}",
        results[0].message
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
#[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
fn skip_entry_parses_old_name_version_format() {
    let deny_content = r#"
[bans]
skip = [
    { name = "windows-sys", version = "0.60.2" },
]
"#;
    let table: toml::Value = deny_content.parse().expect("valid TOML");
    let mut results = Vec::new();
    let path = Path::new("deny.toml");
    check_skip_entries(&table, path, &mut results);
    assert!(!results.is_empty());
    assert!(
        results[0].message.contains("windows-sys"),
        "Should parse name from old format"
    );
    assert!(
        results[0].message.contains("0.60.2"),
        "Should parse version from old format"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
#[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
fn skip_entry_parses_plain_string() {
    let deny_content = r#"
[bans]
skip = ["some-crate"]
"#;
    let table: toml::Value = deny_content.parse().expect("valid TOML");
    let mut results = Vec::new();
    let path = Path::new("deny.toml");
    check_skip_entries(&table, path, &mut results);
    assert!(!results.is_empty());
    assert!(results[0].message.contains("some-crate"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
#[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
fn skip_entry_includes_reason() {
    let deny_content = r#"
[bans]
skip = [
    { crate = "foo@1.0.0", reason = "needed for compat" },
]
"#;
    let table: toml::Value = deny_content.parse().expect("valid TOML");
    let mut results = Vec::new();
    let path = Path::new("deny.toml");
    check_skip_entries(&table, path, &mut results);
    assert!(!results.is_empty());
    assert!(
        results[0].message.contains("needed for compat"),
        "Should include reason in message"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
fn no_skip_section_produces_no_results() {
    let deny_content = r"
[bans]
deny = []
";
    let table: toml::Value = deny_content.parse().expect("valid TOML");
    let mut results = Vec::new();
    let path = Path::new("deny.toml");
    check_skip_entries(&table, path, &mut results);
    assert!(results.is_empty());
}
