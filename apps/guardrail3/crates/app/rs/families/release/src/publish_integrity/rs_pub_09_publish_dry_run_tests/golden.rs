use super::super::check;
use super::super::copy_fixture;
use super::super::run_family;
use super::super::{crate_facts, crate_input};

#[test]
fn inventories_direct_publish_dry_run_success() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(guardrail3_outbound_traits::CommandRunResult::new(
        true,
        String::new(),
    ));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    guardrail3_app_rs_family_release_assertions::publish_integrity::rs_pub_09_publish_dry_run::assert_passed(
        &results,
        "crates/example/Cargo.toml",
        "x: publish dry-run passed",
    );
}

#[test]
fn inventories_real_publish_dry_run_success_from_richer_fixture() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path(), true);

    guardrail3_app_rs_family_release_assertions::publish_integrity::rs_pub_09_publish_dry_run::assert_passed(
        &results,
        "packages/shared-types/Cargo.toml",
        "shared-types: publish dry-run passed",
    );
}
