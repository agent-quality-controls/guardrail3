use super::super::super::test_support::copy_fixture;
use super::super::super::test_support::run_family;
use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_direct_publish_dry_run_success() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(guardrail3_outbound_traits::CommandRunResult {
        success: true,
        stderr: String::new(),
    });
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-09");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "x: publish dry-run passed");
    assert_eq!(results[0].message, "`cargo publish --dry-run` succeeded.");
}

#[test]
fn inventories_real_publish_dry_run_success_from_richer_fixture() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path(), true);
    let shared_types = results
        .iter()
        .find(|result| {
            result.id == "RS-PUB-09"
                && result.inventory
                && result.file.as_deref() == Some("packages/shared-types/Cargo.toml")
        })
        .expect("expected RS-PUB-09 inventory for shared-types");

    assert_eq!(shared_types.title, "shared-types: publish dry-run passed");
    assert_eq!(shared_types.message, "`cargo publish --dry-run` succeeded.");
}
