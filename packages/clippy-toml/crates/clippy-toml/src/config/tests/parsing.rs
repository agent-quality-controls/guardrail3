use crate::config::ClippyConfig;
use toml::Value;

fn parse(input: &str) -> ClippyConfig {
    input.parse::<ClippyConfig>().expect("should parse valid clippy.toml")
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse("");

    assert_eq!(cfg.max_struct_bools, None, "max_struct_bools should be None");
    assert_eq!(cfg.cognitive_complexity_threshold, None, "threshold should be None");
    assert!(cfg.disallowed_methods.is_empty(), "disallowed_methods should be empty");
    assert!(cfg.disallowed_types.is_empty(), "disallowed_types should be empty");
    assert!(cfg.disallowed_macros.is_empty(), "disallowed_macros should be empty");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

#[test]
fn thresholds_parse() {
    let cfg = parse(r#"
max-struct-bools = 3
max-fn-params-bools = 3
too-many-lines-threshold = 75
too-many-arguments-threshold = 7
excessive-nesting-threshold = 4
cognitive-complexity-threshold = 15
type-complexity-threshold = 75
"#);

    assert_eq!(cfg.max_struct_bools, Some(3), "max_struct_bools mismatch");
    assert_eq!(cfg.max_fn_params_bools, Some(3), "max_fn_params_bools mismatch");
    assert_eq!(cfg.too_many_lines_threshold, Some(75), "too_many_lines mismatch");
    assert_eq!(cfg.too_many_arguments_threshold, Some(7), "too_many_arguments mismatch");
    assert_eq!(cfg.excessive_nesting_threshold, Some(4), "excessive_nesting mismatch");
    assert_eq!(cfg.cognitive_complexity_threshold, Some(15), "cognitive_complexity mismatch");
    assert_eq!(cfg.type_complexity_threshold, Some(75), "type_complexity mismatch");
}

#[test]
fn simple_ban_entries() {
    let cfg = parse(r#"
disallowed-methods = ["std::env::var", "std::process::exit"]
disallowed-types = ["std::collections::HashMap"]
disallowed-macros = ["println!", "dbg!"]
"#);

    assert_eq!(cfg.disallowed_methods.len(), 2, "should have 2 method bans");
    assert_eq!(cfg.disallowed_methods[0].path(), "std::env::var", "first method path");
    assert_eq!(cfg.disallowed_methods[0].reason(), None, "simple entry has no reason");

    assert_eq!(cfg.disallowed_types.len(), 1, "should have 1 type ban");
    assert_eq!(cfg.disallowed_macros.len(), 2, "should have 2 macro bans");
}

#[test]
fn detailed_ban_entries_with_reason() {
    let cfg = parse(r#"
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    "std::process::exit",
]
"#);

    assert_eq!(cfg.disallowed_methods.len(), 2, "should have 2 entries");
    assert_eq!(cfg.disallowed_methods[0].path(), "std::env::var", "detailed path");
    assert_eq!(cfg.disallowed_methods[0].reason(), Some("Use config module"), "detailed reason");
    assert_eq!(cfg.disallowed_methods[1].path(), "std::process::exit", "simple path");
    assert_eq!(cfg.disallowed_methods[1].reason(), None, "simple has no reason");
}

#[test]
fn test_relaxations_parse() {
    let cfg = parse(r#"
allow-dbg-in-tests = false
allow-print-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-unwrap-in-tests = false
"#);

    assert_eq!(cfg.allow_dbg_in_tests, Some(false), "allow_dbg mismatch");
    assert_eq!(cfg.allow_print_in_tests, Some(false), "allow_print mismatch");
    assert_eq!(cfg.allow_expect_in_tests, Some(true), "allow_expect mismatch");
    assert_eq!(cfg.allow_panic_in_tests, Some(false), "allow_panic mismatch");
    assert_eq!(cfg.allow_unwrap_in_tests, Some(false), "allow_unwrap mismatch");
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse(r#"
max-struct-bools = 3
some-future-clippy-option = "yes"
"#);

    assert_eq!(cfg.max_struct_bools, Some(3), "known key should parse");
    assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown key");
    assert_eq!(
        cfg.extra.get("some-future-clippy-option").and_then(Value::as_str),
        Some("yes"),
        "unknown key should be captured",
    );
}

#[test]
fn real_clippy_toml_parses() {
    let cfg = parse(r#"
too-many-lines-threshold = 75
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 7
type-complexity-threshold = 75
max-struct-bools = 3
max-fn-params-bools = 3
excessive-nesting-threshold = 4
avoid-breaking-exported-api = false
allow-dbg-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-print-in-tests = false
allow-unwrap-in-tests = false
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    { path = "std::process::exit", reason = "Use error propagation" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "Use BTreeMap" },
]
disallowed-macros = [
    { path = "println", reason = "Use tracing" },
]
"#);

    assert_eq!(cfg.max_struct_bools, Some(3), "threshold mismatch");
    assert_eq!(cfg.avoid_breaking_exported_api, Some(false), "api flag mismatch");
    assert_eq!(cfg.disallowed_methods.len(), 2, "method ban count");
    assert_eq!(cfg.disallowed_types.len(), 1, "type ban count");
    assert_eq!(cfg.disallowed_macros.len(), 1, "macro ban count");
    assert!(cfg.extra.is_empty(), "all keys should be known");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = bad.parse::<ClippyConfig>();
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assert!(
        msg.contains("invalid clippy.toml"),
        "expected error message prefix, got: {msg}",
    );
}
