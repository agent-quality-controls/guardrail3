use nextest_toml_parser_assertions::parser as assertions;
use toml::Value;

use super::helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_empty_toml(&cfg);
}

#[test]
fn single_profile_with_simple_timeouts() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = "60s"
leak-timeout = "100ms"
"#);

    assertions::assert_profile_len(&cfg, 1);
    let default = cfg.profile.get("default").expect("should have 'default' profile");

    assertions::assert_simple_timeout(default.slow_timeout(), "60s", "slow_timeout");
    assertions::assert_simple_timeout(default.leak_timeout(), "100ms", "leak_timeout");
}

#[test]
fn detailed_timeout_with_terminate_after() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");

    assertions::assert_detailed_timeout(default.slow_timeout(), "60s", Some(2));
}

#[test]
fn profile_with_all_known_fields() {
    let cfg = parse_fixture(r#"
[profile.ci]
slow-timeout = { period = "120s", terminate-after = 3 }
leak-timeout = "500ms"
test-threads = 4
retries = 2
fail-fast = false
"#);

    let ci = cfg.profile.get("ci").expect("should have 'ci' profile");

    assert!(ci.slow_timeout().is_some(), "slow_timeout should be present");
    assert!(ci.leak_timeout().is_some(), "leak_timeout should be present");
    assert!(ci.test_threads().is_some(), "test_threads should be present");
    assert!(ci.retries().is_some(), "retries should be present");
    assert_eq!(ci.fail_fast(), Some(false), "fail_fast should be false");
    assertions::assert_profile_extra_empty(ci);
}

#[test]
fn multiple_profiles() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = "60s"

[profile.ci]
slow-timeout = "120s"
fail-fast = true
"#);

    assert_eq!(cfg.profile.len(), 2, "should have 2 profiles");
    assert!(cfg.profile.contains_key("default"), "should have 'default'");
    assert!(cfg.profile.contains_key("ci"), "should have 'ci'");
}

#[test]
fn unknown_top_level_keys_land_in_extra() {
    let cfg = parse_fixture(r#"
some-future-option = "yes"

[profile.default]
slow-timeout = "60s"
"#);

    assertions::assert_top_level_extra_string(&cfg, "some-future-option", "yes");
}

#[test]
fn unknown_profile_keys_land_in_profile_extra() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = "60s"
some-new-nextest-option = true
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
    assertions::assert_profile_extra_bool(default, "some-new-nextest-option", true);
}

#[test]
fn test_threads_as_string() {
    let cfg = parse_fixture(r#"
[profile.default]
test-threads = "num-cpus"
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
    assert!(default.test_threads().is_some(), "test_threads should be present");
    assert_eq!(
        default.test_threads().and_then(Value::as_str),
        Some("num-cpus"),
        "test_threads string value",
    );
}

#[test]
fn real_nextest_config_parses() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
leak-timeout = "100ms"
test-threads = "num-cpus"
retries = 0
fail-fast = true

[profile.ci]
slow-timeout = { period = "120s", terminate-after = 3 }
leak-timeout = "500ms"
retries = 2
fail-fast = false
"#);

    assertions::assert_profile_len(&cfg, 2);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
    assert!(default.slow_timeout().is_some(), "default slow_timeout");
    assert!(default.leak_timeout().is_some(), "default leak_timeout");
    assert_eq!(default.fail_fast(), Some(true), "default fail_fast");
    assertions::assert_profile_extra_empty(default);

    let ci = cfg.profile.get("ci").expect("should have 'ci' profile");
    assert!(ci.slow_timeout().is_some(), "ci slow_timeout");
    assert_eq!(ci.fail_fast(), Some(false), "ci fail_fast");
    assertions::assert_profile_extra_empty(ci);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(r#"
[profile.default]
slow-timeout = { period = "99s", terminate-after = 2 }
leak-timeout = "250ms"
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
    assertions::assert_detailed_timeout(default.slow_timeout(), "99s", Some(2));
    assertions::assert_simple_timeout(default.leak_timeout(), "250ms", "leak_timeout");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = assertions::parse_error(bad);
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assertions::assert_parse_error_message(&msg);
}
