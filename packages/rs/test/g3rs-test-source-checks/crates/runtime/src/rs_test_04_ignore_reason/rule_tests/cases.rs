use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_04_ignore_reason::rule as assertions;

#[test]
fn reports_missing_ignore_reason() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/slow.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore]\nfn slow() {}\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Error,
        "ignored test lacks reason",
        "tests/slow.rs",
        Some(2),
    );
}

#[test]
fn inventories_clean_file_without_ignored_tests() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/ok.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn ok() {}\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-04",
        "ignored tests have reasons",
        "tests/ok.rs",
    );
}

#[test]
fn accepts_previous_line_and_inline_ignore_reasons() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/documented.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n// reason: flakes only under qemu with debug allocator\n#[ignore]\nfn previous_line() {}\n\n#[test]\n#[ignore] // reason: blocked by upstream tokio race in CI only\nfn same_line() {}\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/documented.rs",
        Some(3),
    );
    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/documented.rs",
        Some(7),
    );
    assertions::assert_title_count(&results, "RS-TEST-SOURCE-04", "ignored test count", 1);
}

#[test]
fn accepts_ignore_string_argument_and_cfg_attr_ignore() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/attrs.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore = \"exercises flaky upstream TCP timing only in CI\"]\nfn attr_reason() {}\n\n#[test]\n#[cfg_attr(test, ignore)] // reason: pending deterministic fixture rewrite for this harness\nfn cfg_attr_reason() {}\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/attrs.rs",
        Some(2),
    );
    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/attrs.rs",
        Some(6),
    );
}

#[test]
fn reports_weak_ignore_reason_and_aggregate_counts() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/mixed.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore] // reason: temporary\nfn weak() {}\n\n#[test]\n#[ignore]\nfn missing() {}\n\n#[test]\n#[ignore = \"fails only under miri because allocator layout differs\"]\nfn strong() {}\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Error,
        "ignored test reason too weak",
        "tests/mixed.rs",
        Some(2),
    );
    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Error,
        "ignored test lacks reason",
        "tests/mixed.rs",
        Some(6),
    );
    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/mixed.rs",
        Some(10),
    );

    assertions::assert_message_count(
        &results,
        "RS-TEST-SOURCE-04",
        "ignored test count",
        "`tests/mixed.rs` has 3 ignored tests (1 documented, 1 missing reasons, 1 weak reasons).",
        1,
    );
}
