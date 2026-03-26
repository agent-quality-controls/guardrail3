use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_18_renamed_dependency_direction as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn cross_app_renamed_edge_is_owned_by_rule_24_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nprocessor_alias = { package = \"worker-app-processor\", path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_18 = assertions::errors_by_id(&results, "RS-HEXARCH-18");
    let rule_24 = assertions::errors_by_id(&results, "RS-HEXARCH-24");

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-18").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_24.len(),
        1,
        "rule 24 should own cross-app renamed edges: {rule_24:#?}"
    );
    assert!(
        rule_18.is_empty(),
        "rule 18 should stay out of cross-app renamed ownership: {rule_18:#?}"
    );
}

#[test]
fn inherited_renamed_path_dep_is_owned_by_rule_17_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n  \"crates/app/*\",\n  \"crates/domain/*\",\n  \"crates/ports/inbound/*\",\n  \"crates/ports/outbound/*\",\n  \"crates/adapters/inbound/*\",\n  \"crates/adapters/outbound/*\",\n]\n[workspace.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"crates/adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nqueue_alias = { workspace = true }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_17 = assertions::errors_by_id(&results, "RS-HEXARCH-17");
    let rule_18 = assertions::errors_by_id(&results, "RS-HEXARCH-18");

    assert_eq!(
        rule_17.len(),
        1,
        "rule 17 should own inherited renamed path deps: {rule_17:#?}"
    );
    assert!(
        rule_18.is_empty(),
        "rule 18 should not double-report inherited renamed path deps: {rule_18:#?}"
    );
}

#[test]
fn renamed_dev_dependency_is_owned_by_rule_20_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_18 = assertions::errors_by_id(&results, "RS-HEXARCH-18");
    let rule_20 = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-20")
        .collect::<Vec<_>>();

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-18").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_20.len(),
        1,
        "rule 20 should own renamed dev-dependency direction violations: {rule_20:#?}"
    );
    assert!(
        rule_18.is_empty(),
        "rule 18 should stay out of renamed dev-dependency ownership: {rule_18:#?}"
    );
}

#[test]
fn renamed_target_dependency_is_owned_by_rule_25_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[target.'cfg(unix)'.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_18 = assertions::errors_by_id(&results, "RS-HEXARCH-18");
    let rule_25 = assertions::errors_by_id(&results, "RS-HEXARCH-25");

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-18").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_25.len(),
        1,
        "rule 25 should own renamed target-specific dependency violations: {rule_25:#?}"
    );
    assert!(
        rule_18.is_empty(),
        "rule 18 should stay out of renamed target-dependency ownership: {rule_18:#?}"
    );
}
