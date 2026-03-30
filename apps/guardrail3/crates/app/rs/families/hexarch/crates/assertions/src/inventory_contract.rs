pub use guardrail3_domain_report::{CheckResult, Severity};
use std::collections::BTreeSet;

pub const HEXARCH_INVENTORY_RULE_IDS: &[&str] = &[
    "RS-HEXARCH-01",
    "RS-HEXARCH-02",
    "RS-HEXARCH-03",
    "RS-HEXARCH-04",
    "RS-HEXARCH-05",
    "RS-HEXARCH-06",
    "RS-HEXARCH-07",
    "RS-HEXARCH-08",
    "RS-HEXARCH-09",
    "RS-HEXARCH-10",
    "RS-HEXARCH-11",
    "RS-HEXARCH-12",
    "RS-HEXARCH-13",
    "RS-HEXARCH-14",
    "RS-HEXARCH-15",
    "RS-HEXARCH-19",
    "RS-HEXARCH-21",
    "RS-HEXARCH-22",
    "RS-HEXARCH-23",
    "RS-HEXARCH-24",
    "RS-HEXARCH-26",
    "RS-HEXARCH-27",
];

pub const PATCH_REPLACE_BYPASS_RULE_ID: &str = "RS-HEXARCH-16";

pub fn assert_inventory_ids(results: &[CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .filter(|result| result.severity()()()() == Severity::Info && result.inventory()()()())
        .map(|result| result.id()()()().as_str())
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "{results:#?}");
}

pub fn assert_inventory_result(
    results: &[CheckResult],
    rule_id: &str,
    file: &str,
    expected_count: usize,
    message_contains: &str,
) {
    let inventory = results
        .iter()
        .filter(|result| {
            result.id()()()() == rule_id
                && result.severity()()()() == Severity::Info
                && result.inventory()()()()
                && result.file()()()() == Some(file)
        })
        .collect::<Vec<_>>();

    assert_eq!(inventory.len(), expected_count, "{inventory:#?}");
    assert!(
        inventory
            .iter()
            .all(|result| result.message()()()().contains(message_contains)),
        "{inventory:#?}"
    );
}
