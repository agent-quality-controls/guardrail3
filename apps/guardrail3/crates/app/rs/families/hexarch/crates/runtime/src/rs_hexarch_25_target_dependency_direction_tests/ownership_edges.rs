use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_25_target_dependency_direction as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn target_specific_external_same_name_collision_does_not_trigger_direction_rule() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[target.'cfg(unix)'.dependencies]\nbackend-adapters-outbound-postgres = \"1\"\n",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-25").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn broken_target_specific_same_app_path_does_not_trigger_direction_rule() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[target.'cfg(unix)'.dependencies]\nbackend-adapters-outbound-queue = { path = \"../../adapters/outbound/missing\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-25").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn cross_app_target_edge_is_owned_by_rule_24_not_rule_25() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[target.'cfg(unix)'.dependencies]\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_24 = assertions::errors_by_id(&results, "RS-HEXARCH-24");
    let rule_25 = assertions::errors_by_id(&results, "RS-HEXARCH-25");

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-25").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_24.len(),
        1,
        "rule 24 should own cross-app target path deps: {rule_24:#?}"
    );
    assert!(
        rule_25.is_empty(),
        "rule 25 should stay out of cross-app target ownership: {rule_25:#?}"
    );
}
