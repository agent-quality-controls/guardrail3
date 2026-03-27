use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warn => 1,
        Severity::Info => 2,
    }
}

#[test]
fn warns_for_each_enabled_test_relaxation() {
    let tree = root_workspace_tree(
        r#"
allow-dbg-in-tests = true
allow-expect-in-tests = false
allow-print-in-tests = true
allow-unwrap-in-tests = true
"#,
    );
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let mut actual_messages = results
        .iter()
        .map(|result| {
            (
                result.severity,
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();
    actual_messages.sort_by(|left, right| {
        severity_rank(left.0)
            .cmp(&severity_rank(right.0))
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut expected_messages = vec![
        (
            Severity::Warn,
            "clippy test relaxation enabled".to_owned(),
            "`allow-dbg-in-tests = true` relaxes test output discipline.".to_owned(),
        ),
        (
            Severity::Error,
            "clippy test expect policy misconfigured".to_owned(),
            "`allow-expect-in-tests` must be `true` so tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.".to_owned(),
        ),
        (
            Severity::Warn,
            "clippy test relaxation enabled".to_owned(),
            "`allow-print-in-tests = true` relaxes test output discipline.".to_owned(),
        ),
        (
            Severity::Error,
            "clippy test unwrap relaxation enabled".to_owned(),
            "`allow-unwrap-in-tests` must stay `false` so `unwrap()` remains banned in tests.".to_owned(),
        ),
    ];
    expected_messages.sort_by(|left, right| {
        severity_rank(left.0)
            .cmp(&severity_rank(right.0))
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-17"
            && !result.inventory
            && result.file.as_deref() == Some("clippy.toml")
    }));
}
