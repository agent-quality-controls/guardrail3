#![allow(
    clippy::panic,
    reason = "parser tests intentionally prove concrete parser contracts"
)]
use nextest_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = assertions::parse_fixture("");

    assertions::assert_empty_toml(&cfg);
    assertions::assert_empty_sections(&cfg);
}

#[test]
fn single_profile_with_simple_timeouts() {
    let cfg = assertions::parse_fixture(
        r#"
[store]
dir = "target/nextest"

[profile.default]
slow-timeout = "60s"
leak-timeout = "100ms"
"#,
    );

    assertions::assert_profile_len(&cfg, 1);
    assert_eq!(
        cfg.store.as_ref().and_then(|store| store.dir.as_deref()),
        Some("target/nextest")
    );
    let default = cfg
        .profile
        .get("default")
        .expect("should have 'default' profile");

    assertions::assert_simple_timeout(default.slow_timeout.as_ref(), "60s", "slow_timeout");
    assertions::assert_simple_timeout(default.leak_timeout.as_ref(), "100ms", "leak_timeout");
}

#[test]
fn detailed_timeout_with_terminate_after() {
    let cfg = assertions::parse_fixture(
        r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
"#,
    );

    let default = cfg
        .profile
        .get("default")
        .expect("should have 'default' profile");

    assertions::assert_detailed_timeout(default.slow_timeout.as_ref(), "60s", Some(2));
}

#[test]
fn top_level_known_sections_parse_to_typed_shapes() {
    let cfg = assertions::parse_fixture(
        r#"
nextest-version = { required = "0.9.20", recommended = "0.9.30" }
experimental = ["setup-scripts", "wrapper-scripts"]

[test-groups.serial]
max-threads = "num-cpus"

[scripts.setup.seed-db]
command = { command-line = "bin/seed-db", relative-to = "target" }
slow-timeout = { period = "30s", terminate-after = 1, grace-period = "5s" }
capture-stdout = true

[scripts.wrapper.cargo-runner]
command = "bin/wrap-runner"
target-runner = "within-wrapper"
"#,
    );

    assertions::assert_top_level_known_sections(&cfg);
}

#[test]
fn legacy_script_table_parses_when_setup_scripts_are_enabled() {
    let cfg = assertions::parse_fixture(
        r#"
experimental = ["setup-scripts"]

[script.legacy-setup]
command = ["cargo", "run", "--bin", "legacy-setup"]
"#,
    );

    assertions::assert_legacy_setup_script(&cfg);
}

#[test]
fn profile_with_all_known_fields() {
    let cfg = assertions::parse_fixture(
        r#"
[profile.ci]
inherits = "default"
default-filter = "not test(slow)"
slow-timeout = { period = "120s", terminate-after = 3, on-timeout = "pass" }
leak-timeout = { period = "500ms", result = "fail" }
global-timeout = "2h"
test-threads = -1
threads-required = "num-test-threads"
run-extra-args = ["--nocapture"]
retries = { backoff = "fixed", count = 2, delay = "1s", jitter = true }
flaky-result = "fail"
status-level = "all"
final-status-level = "flaky"
failure-output = "immediate-final"
success-output = "never"
fail-fast = { max-fail = 10, terminate = "wait" }
test-group = "@global"

[[profile.ci.overrides]]
filter = "test(integration_test)"
threads-required = 2
run-extra-args = ["--test-threads", "1"]
retries = 2
slow-timeout = "20s"
leak-timeout = "200ms"
test-group = "serial"
success-output = "final"
failure-output = "immediate"

[[profile.ci.overrides]]
platform = { host = "cfg(unix)", target = "aarch64-apple-darwin" }
default-filter = "not test(ignored)"
priority = 5

[[profile.ci.scripts]]
platform = "x86_64-unknown-linux-gnu"
filter = "test(smoke)"
setup = ["seed-db", "prep-cache"]
list-wrapper = "cargo-runner"
run-wrapper = "cargo-runner"

[profile.ci.junit]
path = "junit.xml"
report-name = "nextest-run"
store-success-output = true
store-failure-output = false
flaky-fail-status = "success"

[profile.ci.archive]
include = [{ path = "application-data", relative-to = "target", depth = "infinite", on-missing = "warn" }]

[profile.ci.bench]
global-timeout = "3h"
slow-timeout = { period = "60s", terminate-after = 10, grace-period = "10s" }
"#,
    );

    assertions::assert_full_ci_profile(&cfg);
}

#[test]
fn unknown_top_level_keys_land_in_extra() {
    let cfg = assertions::parse_fixture(
        r#"
some-future-option = "yes"

[profile.default]
slow-timeout = "60s"
"#,
    );

    assertions::assert_top_level_extra_string(&cfg, "some-future-option", "yes");
}

#[test]
fn unknown_profile_keys_land_in_profile_extra() {
    let cfg = assertions::parse_fixture(
        r#"
[profile.default]
slow-timeout = "60s"
some-new-nextest-option = true
"#,
    );

    let default = cfg
        .profile
        .get("default")
        .expect("should have 'default' profile");
    assertions::assert_profile_extra_bool(default, "some-new-nextest-option", true);
}

#[test]
fn test_threads_num_cpus_parses_as_typed_enum() {
    let cfg = assertions::parse_fixture(
        r#"
[profile.default]
test-threads = "num-cpus"
"#,
    );

    assertions::assert_default_profile_test_threads_num_cpus(&cfg);
}

#[test]
fn zero_test_threads_is_rejected() {
    let err = assertions::parse_error(
        r"
[profile.default]
test-threads = 0
",
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn invalid_archive_depth_is_rejected() {
    let err = assertions::parse_error(
        r#"
[profile.default.archive]
include = [{ path = "fixtures", relative-to = "target", depth = "forever" }]
"#,
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn invalid_wrapper_target_runner_is_rejected() {
    let err = assertions::parse_error(
        r#"
experimental = ["wrapper-scripts"]

[scripts.wrapper.cargo-runner]
command = "bin/wrap"
target-runner = "inside-wrapper"
"#,
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn setup_scripts_require_experimental_feature() {
    let err = assertions::parse_error(
        r#"
[scripts.setup.seed-db]
command = "bin/seed-db"
"#,
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("setup scripts require experimental"),
        "expected setup scripts experimental error, got: {message}",
    );
}

#[test]
fn wrapper_scripts_require_experimental_feature() {
    let err = assertions::parse_error(
        r#"
[scripts.wrapper.cargo-runner]
command = "bin/wrap"
"#,
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("wrapper scripts require experimental"),
        "expected wrapper scripts experimental error, got: {message}",
    );
}

#[test]
fn legacy_and_setup_script_tables_cannot_be_mixed() {
    let err = assertions::parse_error(
        r#"
experimental = ["setup-scripts"]

[script.legacy-setup]
command = "bin/legacy"

[scripts.setup.seed-db]
command = "bin/seed-db"
"#,
    );
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("cannot be used together"),
        "expected mixed script table error, got: {message}",
    );
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = assertions::parse_from_tempfile(
        r#"
[profile.default]
slow-timeout = { period = "99s", terminate-after = 2 }
leak-timeout = "250ms"
"#,
    );

    let default = cfg
        .profile
        .get("default")
        .expect("should have 'default' profile");
    assertions::assert_detailed_timeout(default.slow_timeout.as_ref(), "99s", Some(2));
    assertions::assert_simple_timeout(default.leak_timeout.as_ref(), "250ms", "leak_timeout");
}

#[test]
fn from_str_error_on_invalid_toml() {
    let bad = "this is not [[[valid toml";
    let err = assertions::parse_error(bad);
    assert!(err.is_err(), "invalid TOML should produce an error");

    let msg = err.expect_err("should be an error").to_string();
    assertions::assert_parse_error_message(&msg);
}
