use crate::config::RustfmtConfig;

fn parse(input: &str) -> RustfmtConfig {
    input.parse::<RustfmtConfig>().expect("should parse valid rustfmt.toml")
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse("");

    assert_eq!(cfg.max_width, None, "max_width should be None for empty input");
    assert_eq!(cfg.hard_tabs, None, "hard_tabs should be None for empty input");
    assert_eq!(cfg.edition, None, "edition should be None for empty input");
    assert_eq!(cfg.newline_style, None, "newline_style should be None for empty input");
    assert!(cfg.ignore.is_empty(), "ignore should be empty for empty input");
    assert!(cfg.skip_macro_invocations.is_empty(), "skip_macro_invocations should be empty");
    assert!(cfg.extra.is_empty(), "extra should be empty for empty input");
}

#[test]
fn realistic_config_parses_typed_fields() {
    let cfg = parse(r#"
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
merge_derives = true
"#);

    assert_eq!(cfg.max_width, Some(100), "max_width mismatch");
    assert_eq!(cfg.hard_tabs, Some(false), "hard_tabs mismatch");
    assert_eq!(cfg.tab_spaces, Some(4), "tab_spaces mismatch");
    assert_eq!(cfg.edition.as_deref(), Some("2021"), "edition mismatch");
    assert_eq!(cfg.newline_style.as_deref(), Some("Unix"), "newline_style mismatch");
    assert_eq!(cfg.use_small_heuristics.as_deref(), Some("Default"), "use_small_heuristics mismatch");
    assert_eq!(cfg.reorder_imports, Some(true), "reorder_imports mismatch");
    assert_eq!(cfg.merge_derives, Some(true), "merge_derives mismatch");
    assert!(cfg.extra.is_empty(), "known keys should not land in extra");
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse(r#"
max_width = 100
some_future_nightly_option = "yes"
another_unknown = 42
"#);

    assert_eq!(cfg.max_width, Some(100), "known key should still parse");
    assert_eq!(cfg.extra.len(), 2, "should capture 2 unknown keys");
    assert_eq!(
        cfg.extra
            .get("some_future_nightly_option")
            .and_then(toml::Value::as_str),
        Some("yes"),
        "unknown string key should be captured",
    );
    assert_eq!(
        cfg.extra
            .get("another_unknown")
            .and_then(toml::Value::as_integer),
        Some(42),
        "unknown integer key should be captured",
    );
}

#[test]
fn ban_style_entries_roundtrip() {
    let cfg = parse(r#"
max_width = 120
ignore = ["generated.rs", "vendor/"]
skip_macro_invocations = ["bitflags"]
disable_all_formatting = false
"#);

    assert_eq!(cfg.max_width, Some(120), "max_width mismatch");
    assert_eq!(cfg.ignore, vec!["generated.rs", "vendor/"], "ignore list mismatch");
    assert_eq!(cfg.skip_macro_invocations, vec!["bitflags"], "skip_macro_invocations mismatch");
    assert_eq!(cfg.disable_all_formatting, Some(false), "disable_all_formatting mismatch");

    let serialized = toml::to_string(&cfg).expect("serialization should succeed");
    let cfg2 = parse(&serialized);
    assert_eq!(cfg, cfg2, "roundtrip should produce identical config");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = bad.parse::<RustfmtConfig>();
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assert!(
        msg.contains("invalid rustfmt.toml"),
        "expected error message prefix, got: {msg}",
    );
}
