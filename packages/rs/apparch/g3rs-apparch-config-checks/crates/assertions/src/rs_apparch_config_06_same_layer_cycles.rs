use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-06";

pub fn assert_cycle(results: &[G3CheckResult], title_fragment: &str, needle: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains(title_fragment)
                && !result.inventory()
                && result.message().contains(needle)
        }),
        "{results:#?}"
    );
}

pub fn assert_cycle_members(
    results: &[G3CheckResult],
    title: &str,
    member_label: &str,
    expected_members: &[&str],
) {
    let expected_message = format!(
        "Found same-layer dependency cycle among {member_label}: {}. Break the cycle by extracting shared code into one owning crate or removing one dependency edge.",
        expected_members.join(" -> ")
    );
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == title
                && !result.inventory()
                && result.message() == expected_message
        }),
        "{results:#?}"
    );
}

pub fn assert_inventory_checked_nodes(results: &[G3CheckResult], expected_nodes: usize) {
    let expected_message =
        format!("Apparch checked {expected_nodes} same-layer dependency node(s) and found no non-dev cycles.");
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.inventory()
                && result.title() == "no same-layer dependency cycles detected"
                && result.message() == expected_message
        }),
        "{results:#?}"
    );
}

pub fn assert_no_findings(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(|result| result.id() != ID),
        "{results:#?}"
    );
}
