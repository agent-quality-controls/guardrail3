use crate::config::MutantsConfig;
use toml::Value;

fn parse(input: &str) -> MutantsConfig {
    input.parse::<MutantsConfig>().expect("should parse valid mutants.toml")
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse("");

    assert!(cfg.exclude_re.is_empty(), "exclude_re should be empty");
    assert!(cfg.examine_re.is_empty(), "examine_re should be empty");
    assert_eq!(cfg.timeout_multiplier, None, "timeout_multiplier should be None");
    assert_eq!(cfg.test_tool, None, "test_tool should be None");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

#[test]
fn realistic_config_parses_typed_fields() {
    let cfg = parse(r#"
timeout_multiplier = 3.0
minimum_test_timeout = "20s"
test_tool = "nextest"
exclude_re = ["^tests/", "^benches/"]
examine_globs = ["src/**/*.rs"]
"#);

    assert_eq!(cfg.timeout_multiplier, Some(3.0), "timeout_multiplier mismatch");
    assert_eq!(cfg.minimum_test_timeout.as_deref(), Some("20s"), "minimum_test_timeout mismatch");
    assert_eq!(cfg.test_tool.as_deref(), Some("nextest"), "test_tool mismatch");
    assert_eq!(cfg.exclude_re, vec!["^tests/", "^benches/"], "exclude_re mismatch");
    assert_eq!(cfg.examine_globs, vec!["src/**/*.rs"], "examine_globs mismatch");
    assert!(cfg.extra.is_empty(), "known keys should not land in extra");
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse(r#"
timeout_multiplier = 2.0
some_future_option = true
"#);

    assert_eq!(cfg.timeout_multiplier, Some(2.0), "known key should parse");
    assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown key");
    assert_eq!(
        cfg.extra.get("some_future_option").and_then(Value::as_bool),
        Some(true),
        "unknown bool key should be captured",
    );
}

#[test]
fn feature_control_fields() {
    let cfg = parse(r#"
all_features = true
no_default_features = false
features = ["serde", "derive"]
"#);

    assert_eq!(cfg.all_features, Some(true), "all_features mismatch");
    assert_eq!(cfg.no_default_features, Some(false), "no_default_features mismatch");
    assert_eq!(cfg.features, vec!["serde", "derive"], "features mismatch");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = bad.parse::<MutantsConfig>();
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assert!(
        msg.contains("invalid mutants.toml"),
        "expected error message prefix, got: {msg}",
    );
}
