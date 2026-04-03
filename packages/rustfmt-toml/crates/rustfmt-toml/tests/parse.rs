use rustfmt_toml::RustfmtConfig;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = RustfmtConfig::from_str("").unwrap();

    // All typed fields are None / empty.
    assert_eq!(cfg.max_width, None);
    assert_eq!(cfg.hard_tabs, None);
    assert_eq!(cfg.edition, None);
    assert_eq!(cfg.newline_style, None);
    assert!(cfg.ignore.is_empty());
    assert!(cfg.skip_macro_invocations.is_empty());
    assert!(cfg.extra.is_empty());
}

#[test]
fn realistic_config_parses_typed_fields() {
    let input = r#"
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
merge_derives = true
"#;
    let cfg = RustfmtConfig::from_str(input).unwrap();

    assert_eq!(cfg.max_width, Some(100));
    assert_eq!(cfg.hard_tabs, Some(false));
    assert_eq!(cfg.tab_spaces, Some(4));
    assert_eq!(cfg.edition.as_deref(), Some("2021"));
    assert_eq!(cfg.newline_style.as_deref(), Some("Unix"));
    assert_eq!(cfg.use_small_heuristics.as_deref(), Some("Default"));
    assert_eq!(cfg.reorder_imports, Some(true));
    assert_eq!(cfg.merge_derives, Some(true));
    assert!(cfg.extra.is_empty());
}

#[test]
fn unknown_keys_land_in_extra() {
    let input = r#"
max_width = 100
some_future_nightly_option = "yes"
another_unknown = 42
"#;
    let cfg = RustfmtConfig::from_str(input).unwrap();

    assert_eq!(cfg.max_width, Some(100));
    assert_eq!(cfg.extra.len(), 2);
    assert_eq!(
        cfg.extra.get("some_future_nightly_option").and_then(|v| v.as_str()),
        Some("yes"),
    );
    assert_eq!(
        cfg.extra.get("another_unknown").and_then(|v| v.as_integer()),
        Some(42),
    );
}

#[test]
fn ban_style_entries_roundtrip() {
    let input = r#"
max_width = 120
ignore = ["generated.rs", "vendor/"]
skip_macro_invocations = ["bitflags"]
disable_all_formatting = false
"#;
    let cfg = RustfmtConfig::from_str(input).unwrap();

    assert_eq!(cfg.max_width, Some(120));
    assert_eq!(cfg.ignore, vec!["generated.rs", "vendor/"]);
    assert_eq!(cfg.skip_macro_invocations, vec!["bitflags"]);
    assert_eq!(cfg.disable_all_formatting, Some(false));

    // Roundtrip through serialization.
    let serialized = toml::to_string(&cfg).unwrap();
    let cfg2 = RustfmtConfig::from_str(&serialized).unwrap();
    assert_eq!(cfg, cfg2);
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = RustfmtConfig::from_str(bad);
    assert!(err.is_err());

    let msg = err.unwrap_err().to_string();
    assert!(
        msg.contains("invalid rustfmt.toml"),
        "expected error message prefix, got: {msg}",
    );
}
