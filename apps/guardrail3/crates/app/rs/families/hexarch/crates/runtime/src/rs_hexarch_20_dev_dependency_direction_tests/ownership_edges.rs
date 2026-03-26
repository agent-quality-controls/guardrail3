use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_20_dev_dependency_direction as assertions;
use super::{copy_fixture, write_file};

#[test]
fn renamed_dev_edge_is_owned_by_rule_20_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    let rule_18 = assertions::errors_by_id(&results, "RS-HEXARCH-18");
    let rule_20 = results
        .iter()
        .filter(|result| result.id == "")
        .collect::<Vec<_>>();

    assertions::assert_no_error(&results, "RS-HEXARCH-18");
    assert_eq!(
        rule_20.len(),
        1,
        "rule 20 should own renamed dev dependency violations: {rule_20:#?}"
    );
    assert!(
        rule_18.is_empty(),
        "rule 18 should stay out of renamed dev ownership: {rule_18:#?}"
    );
}

#[test]
fn inherited_dev_edge_is_owned_by_rule_20_not_rule_17() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n  \"crates/app/*\",\n  \"crates/domain/*\",\n  \"crates/ports/inbound/*\",\n  \"crates/ports/outbound/*\",\n  \"crates/adapters/inbound/*\",\n  \"crates/adapters/outbound/*\",\n]\n[workspace.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"crates/adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { workspace = true }\n",
    );

    let results = super::run_family(tmp.path());
    let rule_17 = assertions::errors_by_id(&results, "RS-HEXARCH-17");
    let rule_20 = results
        .iter()
        .filter(|result| result.id == "")
        .collect::<Vec<_>>();

    assert!(
        rule_17.is_empty(),
        "rule 17 should stay out of inherited dev-dependency ownership: {rule_17:#?}"
    );
    assert_eq!(
        rule_20.len(),
        1,
        "rule 20 should own inherited dev path dependency violations: {rule_20:#?}"
    );
}
