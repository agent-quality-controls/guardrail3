use guardrail3_domain_report::Severity;

use super::super::{crate_facts, crate_input};
use super::super::check;

#[test]
fn inventories_good_readme_quality_for_publishable_crate() {
    let facts = crate_facts("example");
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].message.contains("content and headings"));
}

#[test]
fn inventories_good_readme_quality_with_setext_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!(
        "Example crate\n=============\n\n{}",
        "x".repeat(260)
    ));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
}

#[test]
fn inventories_good_readme_quality_with_multi_hash_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!("## Example crate\n\n{}", "x".repeat(260)));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
}
