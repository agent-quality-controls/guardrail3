#![allow(
    clippy::indexing_slicing,
    clippy::panic,
    clippy::too_many_lines,
    reason = "parser tests intentionally use direct field assertions to prove file-shape contracts"
)]

use std::collections::BTreeMap;

use crate::{
    ArchiveDepth, ArchiveOnMissing, ExperimentalFeature, FailFastConfig, FailFastCount,
    FailFastDetail, FinalStatusLevel, FlakyResult, JunitFlakyFailStatus, NextestVersionConfig,
    NextestVersionDetail, PlatformConfig, PlatformDetail, ProfileOverride, ProfileScriptConfig,
    RelativeTo, RetryPolicy, RetryPolicyDetail, ScriptCommand, ScriptCommandDetail,
    ScriptReference, StatusLevel, TargetRunnerMode, TerminateMode, TestGroupMaxThreads,
    TestOutputDisplay, TestThreads, ThreadsRequired, TimeoutConfig,
};
use nextest_toml_parser_runtime_assertions::parser as assertions;

use super::helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_empty_toml(&cfg);
    assert!(cfg.experimental.is_empty(), "experimental should be empty");
    assert!(cfg.test_groups.is_empty(), "test groups should be empty");
    assert!(cfg.script.is_empty(), "legacy script table should be empty");
    assert!(cfg.scripts.is_none(), "scripts should be absent");
}

#[test]
fn single_profile_with_simple_timeouts() {
    let cfg = parse_fixture(r#"
[store]
dir = "target/nextest"

[profile.default]
slow-timeout = "60s"
leak-timeout = "100ms"
"#);

    assertions::assert_profile_len(&cfg, 1);
    assert_eq!(
        cfg.store.as_ref().and_then(|store| store.dir.as_deref()),
        Some("target/nextest")
    );
    let default = cfg.profile.get("default").expect("should have 'default' profile");

    assertions::assert_simple_timeout(default.slow_timeout.as_ref(), "60s", "slow_timeout");
    assertions::assert_simple_timeout(default.leak_timeout.as_ref(), "100ms", "leak_timeout");
}

#[test]
fn detailed_timeout_with_terminate_after() {
    let cfg = parse_fixture(r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");

    assertions::assert_detailed_timeout(default.slow_timeout.as_ref(), "60s", Some(2));
}

#[test]
fn top_level_known_sections_parse_to_typed_shapes() {
    let cfg = parse_fixture(r#"
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
"#);

    assert_eq!(
        cfg.nextest_version,
        Some(NextestVersionConfig::Detailed(NextestVersionDetail {
            required: Some("0.9.20".to_owned()),
            recommended: Some("0.9.30".to_owned()),
            extra: BTreeMap::default(),
        }))
    );
    assert_eq!(
        cfg.experimental,
        vec![
            ExperimentalFeature::SetupScripts,
            ExperimentalFeature::WrapperScripts
        ]
    );
    assert_eq!(
        cfg.test_groups["serial"].max_threads,
        Some(TestGroupMaxThreads::NumCpus)
    );
    let scripts = cfg.scripts.as_ref().expect("scripts should be present");
    assert_eq!(
        scripts.setup["seed-db"].command,
        ScriptCommand::Detailed(ScriptCommandDetail {
            command_line: "bin/seed-db".to_owned(),
            relative_to: Some(RelativeTo::Target),
            extra: BTreeMap::default(),
        })
    );
    assert_eq!(scripts.setup["seed-db"].capture_stdout, Some(true));
    assert_eq!(
        scripts.wrapper["cargo-runner"].target_runner,
        Some(TargetRunnerMode::WithinWrapper)
    );
}

#[test]
fn legacy_script_table_parses_when_setup_scripts_are_enabled() {
    let cfg = parse_fixture(r#"
experimental = ["setup-scripts"]

[script.legacy-setup]
command = ["cargo", "run", "--bin", "legacy-setup"]
"#);

    assert_eq!(
        cfg.script["legacy-setup"].command,
        ScriptCommand::Argv(vec![
            "cargo".to_owned(),
            "run".to_owned(),
            "--bin".to_owned(),
            "legacy-setup".to_owned()
        ])
    );
}

#[test]
fn profile_with_all_known_fields() {
    let cfg = parse_fixture(r#"
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
"#);

    let ci = cfg.profile.get("ci").expect("should have 'ci' profile");

    assert_eq!(ci.inherits.as_deref(), Some("default"));
    assert_eq!(ci.default_filter.as_deref(), Some("not test(slow)"));
    assertions::assert_detailed_timeout(ci.slow_timeout.as_ref(), "120s", Some(3));
    assert!(ci.leak_timeout.is_some(), "leak_timeout should be present");
    assert_eq!(ci.global_timeout.as_deref(), Some("2h"));
    assertions::assert_test_threads(ci.test_threads, TestThreads::Count(-1));
    assertions::assert_threads_required(ci.threads_required, ThreadsRequired::NumTestThreads);
    assert_eq!(ci.run_extra_args, ["--nocapture"]);
    assert_eq!(
        ci.retries,
        Some(RetryPolicy::Fixed(RetryPolicyDetail {
            count: 2,
            delay: Some("1s".to_owned()),
            jitter: true,
        }))
    );
    assert_eq!(ci.flaky_result, Some(FlakyResult::Fail));
    assert_eq!(ci.status_level, Some(StatusLevel::All));
    assert_eq!(ci.final_status_level, Some(FinalStatusLevel::Flaky));
    assert_eq!(ci.failure_output, Some(TestOutputDisplay::ImmediateFinal));
    assert_eq!(ci.success_output, Some(TestOutputDisplay::Never));
    assert_eq!(
        ci.fail_fast,
        Some(FailFastConfig::Detailed(FailFastDetail {
            max_fail: FailFastCount::Count(10),
            terminate: Some(TerminateMode::Wait),
        }))
    );
    assert_eq!(ci.test_group.as_deref(), Some("@global"));
    assert_eq!(ci.overrides.len(), 2);
    assert_eq!(
        ci.overrides[0],
        ProfileOverride {
            filter: Some("test(integration_test)".to_owned()),
            platform: None,
            default_filter: None,
            priority: None,
            threads_required: Some(ThreadsRequired::Count(2)),
            run_extra_args: vec!["--test-threads".to_owned(), "1".to_owned()],
            retries: Some(RetryPolicy::Count(2)),
            flaky_result: None,
            slow_timeout: Some(TimeoutConfig::Simple("20s".to_owned())),
            bench: None,
            leak_timeout: Some(TimeoutConfig::Simple("200ms".to_owned())),
            test_group: Some("serial".to_owned()),
            success_output: Some(TestOutputDisplay::Final),
            failure_output: Some(TestOutputDisplay::Immediate),
            junit: None,
            extra: BTreeMap::default(),
        }
    );
    assert_eq!(
        ci.overrides[1].platform,
        Some(PlatformConfig::Detailed(PlatformDetail {
            host: Some("cfg(unix)".to_owned()),
            target: Some("aarch64-apple-darwin".to_owned()),
            extra: BTreeMap::default(),
        }))
    );
    assert_eq!(ci.overrides[1].default_filter.as_deref(), Some("not test(ignored)"));
    assert_eq!(ci.overrides[1].priority, Some(5));
    assert_eq!(
        ci.scripts[0],
        ProfileScriptConfig {
            platform: Some(PlatformConfig::Name("x86_64-unknown-linux-gnu".to_owned())),
            filter: Some("test(smoke)".to_owned()),
            setup: Some(ScriptReference::Multiple(vec![
                "seed-db".to_owned(),
                "prep-cache".to_owned()
            ])),
            list_wrapper: Some("cargo-runner".to_owned()),
            run_wrapper: Some("cargo-runner".to_owned()),
            extra: BTreeMap::default(),
        }
    );
    let junit = ci.junit.as_ref().expect("junit should be present");
    assert_eq!(junit.path.as_deref(), Some("junit.xml"));
    assert_eq!(junit.report_name.as_deref(), Some("nextest-run"));
    assert_eq!(junit.store_success_output, Some(true));
    assert_eq!(junit.store_failure_output, Some(false));
    assert_eq!(junit.flaky_fail_status, Some(JunitFlakyFailStatus::Success));
    let archive = ci.archive.as_ref().expect("archive should be present");
    assert_eq!(archive.include.len(), 1);
    assert_eq!(archive.include[0].path, "application-data");
    assert_eq!(archive.include[0].relative_to, Some(RelativeTo::Target));
    assert_eq!(archive.include[0].depth, Some(ArchiveDepth::Infinite));
    assert_eq!(archive.include[0].on_missing, Some(ArchiveOnMissing::Warn));
    let bench = ci.bench.as_ref().expect("bench should be present");
    assert_eq!(bench.global_timeout.as_deref(), Some("3h"));
    assertions::assert_detailed_timeout(bench.slow_timeout.as_ref(), "60s", Some(10));
    assertions::assert_profile_extra_empty(ci);
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
fn test_threads_num_cpus_parses_as_typed_enum() {
    let cfg = parse_fixture(r#"
[profile.default]
test-threads = "num-cpus"
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
    assertions::assert_test_threads(default.test_threads, TestThreads::NumCpus);
}

#[test]
fn zero_test_threads_is_rejected() {
    let err = assertions::parse_error(r"
[profile.default]
test-threads = 0
");
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn invalid_archive_depth_is_rejected() {
    let err = assertions::parse_error(r#"
[profile.default.archive]
include = [{ path = "fixtures", relative-to = "target", depth = "forever" }]
"#);
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn invalid_wrapper_target_runner_is_rejected() {
    let err = assertions::parse_error(r#"
experimental = ["wrapper-scripts"]

[scripts.wrapper.cargo-runner]
command = "bin/wrap"
target-runner = "inside-wrapper"
"#);
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
}

#[test]
fn setup_scripts_require_experimental_feature() {
    let err = assertions::parse_error(r#"
[scripts.setup.seed-db]
command = "bin/seed-db"
"#);
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("setup scripts require experimental"),
        "expected setup scripts experimental error, got: {message}",
    );
}

#[test]
fn wrapper_scripts_require_experimental_feature() {
    let err = assertions::parse_error(r#"
[scripts.wrapper.cargo-runner]
command = "bin/wrap"
"#);
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("wrapper scripts require experimental"),
        "expected wrapper scripts experimental error, got: {message}",
    );
}

#[test]
fn legacy_and_setup_script_tables_cannot_be_mixed() {
    let err = assertions::parse_error(r#"
experimental = ["setup-scripts"]

[script.legacy-setup]
command = "bin/legacy"

[scripts.setup.seed-db]
command = "bin/seed-db"
"#);
    let message = err.expect_err("should be an error").to_string();

    assertions::assert_parse_error_message(&message);
    assert!(
        message.contains("cannot be used together"),
        "expected mixed script table error, got: {message}",
    );
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(r#"
[profile.default]
slow-timeout = { period = "99s", terminate-after = 2 }
leak-timeout = "250ms"
"#);

    let default = cfg.profile.get("default").expect("should have 'default' profile");
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
