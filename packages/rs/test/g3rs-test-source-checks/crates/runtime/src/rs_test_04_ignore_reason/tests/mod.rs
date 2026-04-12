use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_missing_ignore_reason() {
    let results = run_input(input(
        vec![file(
            "tests/slow.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore]\nfn slow() {}\n",
        )],
        None,
    ));

    assert_has_result(
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
    let results = run_input(input(
        vec![file(
            "tests/ok.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn ok() {}\n",
        )],
        None,
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-04",
        "ignored tests have reasons",
        "tests/ok.rs",
    );
}

#[test]
fn accepts_previous_line_and_inline_ignore_reasons() {
    let results = run_input(input(
        vec![file(
            "tests/documented.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n// reason: flakes only under qemu with debug allocator\n#[ignore]\nfn previous_line() {}\n\n#[test]\n#[ignore] // reason: blocked by upstream tokio race in CI only\nfn same_line() {}\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/documented.rs",
        Some(3),
    );
    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/documented.rs",
        Some(7),
    );
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TEST-SOURCE-04" && result.title() == "ignored test count")
            .count(),
        1,
        "{results:#?}"
    );
}

#[test]
fn accepts_ignore_string_argument_and_cfg_attr_ignore() {
    let results = run_input(input(
        vec![file(
            "tests/attrs.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore = \"exercises flaky upstream TCP timing only in CI\"]\nfn attr_reason() {}\n\n#[test]\n#[cfg_attr(test, ignore)] // reason: pending deterministic fixture rewrite for this harness\nfn cfg_attr_reason() {}\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/attrs.rs",
        Some(2),
    );
    assert_has_result(
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
    let results = run_input(input(
        vec![file(
            "tests/mixed.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore] // reason: temporary\nfn weak() {}\n\n#[test]\n#[ignore]\nfn missing() {}\n\n#[test]\n#[ignore = \"fails only under miri because allocator layout differs\"]\nfn strong() {}\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Error,
        "ignored test reason too weak",
        "tests/mixed.rs",
        Some(2),
    );
    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Error,
        "ignored test lacks reason",
        "tests/mixed.rs",
        Some(6),
    );
    assert_has_result(
        &results,
        "RS-TEST-SOURCE-04",
        G3Severity::Warn,
        "ignored test has documented reason",
        "tests/mixed.rs",
        Some(10),
    );

    let count_results = results
        .iter()
        .filter(|result| {
            result.id() == "RS-TEST-SOURCE-04"
                && result.title() == "ignored test count"
                && result.message()
                    == "`tests/mixed.rs` has 3 ignored tests (1 documented, 1 missing reasons, 1 weak reasons)."
        })
        .count();
    assert_eq!(count_results, 1, "{results:#?}");
}
