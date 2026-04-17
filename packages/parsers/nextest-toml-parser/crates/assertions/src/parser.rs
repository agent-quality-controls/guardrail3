#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use std::collections::BTreeMap;
use std::io::Write;

use nextest_toml_parser_runtime::Value;
use nextest_toml_parser_runtime::types;
use nextest_toml_parser_runtime::types::nextest_toml::{basics, execution, profile, scripts};

pub fn parse_fixture(input: &str) -> types::NextestToml {
    nextest_toml_parser_runtime::parse(input).expect("should parse valid nextest.toml")
}

pub fn parse_from_tempfile(input: &str) -> types::NextestToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("nextest config should be written");
    nextest_toml_parser_runtime::from_path(file.path()).expect("file should parse")
}

pub fn assert_empty_toml(cfg: &types::NextestToml) {
    assert!(cfg.profile.is_empty(), "profile map should be empty");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_empty_sections(cfg: &types::NextestToml) {
    assert!(cfg.experimental.is_empty(), "experimental should be empty");
    assert!(cfg.test_groups.is_empty(), "test groups should be empty");
    assert!(cfg.script.is_empty(), "legacy script table should be empty");
    assert!(cfg.scripts.is_none(), "scripts should be absent");
}

pub fn assert_profile_len(cfg: &types::NextestToml, expected: usize) {
    assert_eq!(cfg.profile.len(), expected, "profile map length mismatch");
}

pub fn assert_profile_extra_empty(profile: &profile::NextestProfile) {
    assert!(profile.extra.is_empty(), "profile extra should be empty");
}

pub fn assert_top_level_extra_string(cfg: &types::NextestToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_profile_extra_bool(profile: &profile::NextestProfile, key: &str, expected: bool) {
    assert_eq!(
        profile.extra.get(key).and_then(Value::as_bool),
        Some(expected),
        "profile extra key should be preserved",
    );
}

pub fn assert_test_threads(
    actual: Option<execution::TestThreads>,
    expected: execution::TestThreads,
) {
    assert_eq!(actual, Some(expected), "test-threads mismatch");
}

pub fn assert_threads_required(
    actual: Option<execution::ThreadsRequired>,
    expected: execution::ThreadsRequired,
) {
    assert_eq!(actual, Some(expected), "threads-required mismatch");
}

pub fn assert_simple_timeout(
    actual: Option<&execution::TimeoutConfig>,
    expected: &str,
    field_name: &str,
) {
    assert!(
        matches!(actual, Some(execution::TimeoutConfig::Simple(s)) if s == expected),
        "expected Simple timeout for {field_name}, got: {actual:?}",
    );
}

pub fn assert_detailed_timeout(
    actual: Option<&execution::TimeoutConfig>,
    period: &str,
    terminate_after: Option<u32>,
) {
    assert!(
        matches!(actual, Some(execution::TimeoutConfig::Detailed(detail))
            if detail.period == period
                && detail.terminate_after == terminate_after
                && detail.extra.is_empty()),
        "expected Detailed timeout, got: {actual:?}",
    );
}

pub fn assert_parse_error_message(message: &str) {
    assert!(
        message.contains("invalid nextest.toml"),
        "expected error message prefix, got: {message}",
    );
}

pub fn assert_top_level_known_sections(cfg: &types::NextestToml) {
    assert_eq!(
        cfg.nextest_version,
        Some(basics::NextestVersionConfig::Detailed(
            basics::NextestVersionDetail {
                required: Some("0.9.20".to_owned()),
                recommended: Some("0.9.30".to_owned()),
                extra: BTreeMap::default(),
            }
        ))
    );
    assert_eq!(
        cfg.experimental,
        vec![
            basics::ExperimentalFeature::SetupScripts,
            basics::ExperimentalFeature::WrapperScripts,
        ]
    );
    assert_eq!(
        cfg.test_groups["serial"].max_threads,
        Some(execution::TestGroupMaxThreads::NumCpus)
    );
    let scripts = cfg.scripts.as_ref().expect("scripts should be present");
    assert_eq!(
        scripts.setup["seed-db"].command,
        scripts::ScriptCommand::Detailed(scripts::ScriptCommandDetail {
            command_line: "bin/seed-db".to_owned(),
            relative_to: Some(execution::RelativeTo::Target),
            extra: BTreeMap::default(),
        })
    );
    assert_eq!(scripts.setup["seed-db"].capture_stdout, Some(true));
    assert_eq!(
        scripts.wrapper["cargo-runner"].target_runner,
        Some(scripts::TargetRunnerMode::WithinWrapper)
    );
}

pub fn assert_legacy_setup_script(cfg: &types::NextestToml) {
    assert_eq!(
        cfg.script["legacy-setup"].command,
        scripts::ScriptCommand::Argv(vec![
            "cargo".to_owned(),
            "run".to_owned(),
            "--bin".to_owned(),
            "legacy-setup".to_owned(),
        ])
    );
}

pub fn assert_full_ci_profile(cfg: &types::NextestToml) {
    let ci = cfg.profile.get("ci").expect("should have 'ci' profile");

    assert_eq!(ci.inherits.as_deref(), Some("default"));
    assert_eq!(ci.default_filter.as_deref(), Some("not test(slow)"));
    assert_detailed_timeout(ci.slow_timeout.as_ref(), "120s", Some(3));
    assert!(ci.leak_timeout.is_some(), "leak_timeout should be present");
    assert_eq!(ci.global_timeout.as_deref(), Some("2h"));
    assert_test_threads(ci.test_threads, execution::TestThreads::Count(-1));
    assert_threads_required(
        ci.threads_required,
        execution::ThreadsRequired::NumTestThreads,
    );
    assert_eq!(ci.run_extra_args, ["--nocapture"]);
    assert_eq!(
        ci.retries,
        Some(execution::RetryPolicy::Fixed(
            execution::RetryPolicyDetail {
                count: 2,
                delay: Some("1s".to_owned()),
                jitter: true,
            }
        ))
    );
    assert_eq!(ci.flaky_result, Some(execution::FlakyResult::Fail));
    assert_eq!(ci.status_level, Some(execution::StatusLevel::All));
    assert_eq!(
        ci.final_status_level,
        Some(execution::FinalStatusLevel::Flaky)
    );
    assert_eq!(
        ci.failure_output,
        Some(execution::TestOutputDisplay::ImmediateFinal)
    );
    assert_eq!(ci.success_output, Some(execution::TestOutputDisplay::Never));
    assert_eq!(
        ci.fail_fast,
        Some(execution::FailFastConfig::Detailed(
            execution::FailFastDetail {
                max_fail: execution::FailFastCount::Count(10),
                terminate: Some(execution::TerminateMode::Wait),
            }
        ))
    );
    assert_eq!(ci.test_group.as_deref(), Some("@global"));
    assert_eq!(ci.overrides.len(), 2);
    assert_eq!(
        ci.overrides[0],
        profile::ProfileOverride {
            filter: Some("test(integration_test)".to_owned()),
            platform: None,
            default_filter: None,
            priority: None,
            threads_required: Some(execution::ThreadsRequired::Count(2)),
            run_extra_args: vec!["--test-threads".to_owned(), "1".to_owned()],
            retries: Some(execution::RetryPolicy::Count(2)),
            flaky_result: None,
            slow_timeout: Some(execution::TimeoutConfig::Simple("20s".to_owned())),
            bench: None,
            leak_timeout: Some(execution::TimeoutConfig::Simple("200ms".to_owned())),
            test_group: Some("serial".to_owned()),
            success_output: Some(execution::TestOutputDisplay::Final),
            failure_output: Some(execution::TestOutputDisplay::Immediate),
            junit: None,
            extra: BTreeMap::default(),
        }
    );
    assert_eq!(
        ci.overrides[1].platform,
        Some(profile::PlatformConfig::Detailed(profile::PlatformDetail {
            host: Some("cfg(unix)".to_owned()),
            target: Some("aarch64-apple-darwin".to_owned()),
            extra: BTreeMap::default(),
        }))
    );
    assert_eq!(
        ci.overrides[1].default_filter.as_deref(),
        Some("not test(ignored)")
    );
    assert_eq!(ci.overrides[1].priority, Some(5));
    assert_eq!(
        ci.scripts[0],
        profile::ProfileScriptConfig {
            platform: Some(profile::PlatformConfig::Name(
                "x86_64-unknown-linux-gnu".to_owned(),
            )),
            filter: Some("test(smoke)".to_owned()),
            setup: Some(profile::ScriptReference::Multiple(vec![
                "seed-db".to_owned(),
                "prep-cache".to_owned(),
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
    assert_eq!(
        junit.flaky_fail_status,
        Some(profile::JunitFlakyFailStatus::Success)
    );
    let archive = ci.archive.as_ref().expect("archive should be present");
    assert_eq!(archive.include.len(), 1);
    assert_eq!(archive.include[0].path, "application-data");
    assert_eq!(
        archive.include[0].relative_to,
        Some(execution::RelativeTo::Target)
    );
    assert_eq!(
        archive.include[0].depth,
        Some(execution::ArchiveDepth::Infinite)
    );
    assert_eq!(
        archive.include[0].on_missing,
        Some(execution::ArchiveOnMissing::Warn)
    );
    let bench = ci.bench.as_ref().expect("bench should be present");
    assert_eq!(bench.global_timeout.as_deref(), Some("3h"));
    assert_detailed_timeout(bench.slow_timeout.as_ref(), "60s", Some(10));
    assert_profile_extra_empty(ci);
}

pub fn assert_default_profile_test_threads_num_cpus(cfg: &types::NextestToml) {
    let default = cfg
        .profile
        .get("default")
        .expect("should have 'default' profile");
    assert_test_threads(default.test_threads, execution::TestThreads::NumCpus);
}

/// Parse nextest TOML content through the runtime parser.
///
/// # Errors
///
/// Returns the parser error when the input is not valid nextest TOML.
pub fn parse_error(input: &str) -> Result<types::NextestToml, nextest_toml_parser_runtime::Error> {
    nextest_toml_parser_runtime::parse(input)
}
