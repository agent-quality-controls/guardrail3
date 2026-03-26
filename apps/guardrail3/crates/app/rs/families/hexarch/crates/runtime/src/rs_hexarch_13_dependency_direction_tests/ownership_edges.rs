use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_13_dependency_direction as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn cross_app_normal_edge_is_owned_by_rule_24_not_rule_13() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_13 = assertions::errors_by_id(&results, "RS-HEXARCH-13");
    let rule_24 = assertions::errors_by_id(&results, "RS-HEXARCH-24");

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-13").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_24.len(),
        1,
        "rule 24 should own cross-app normal edges: {rule_24:#?}"
    );
    assert!(
        rule_13.is_empty(),
        "rule 13 should stay out of cross-app ownership: {rule_13:#?}"
    );
}
