use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_21_domain_purity as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn out_of_tree_paths_with_pure_layer_names_still_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[dependencies]\nvendor-domain-kit = { path = \"../../../../../vendor/domain/kit\" }\nvendor-ports-kit = { path = \"../../../../../vendor/ports/kit\" }\n",
    );
    write_file(
        tmp.path(),
        "vendor/domain/kit/Cargo.toml",
        "[package]\nname = \"vendor-domain-kit\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "vendor/ports/kit/Cargo.toml",
        "[package]\nname = \"vendor-ports-kit\"\nversion = \"0.1.0\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let actual_titles = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-21")
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "domain crate `backend-domain-engine` depends on disallowed external crate `vendor-domain-kit`"
            .to_owned(),
        "domain crate `backend-domain-engine` depends on disallowed external crate `vendor-ports-kit`"
            .to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_titles, expected_titles,
        "out-of-tree path deps must not be treated as pure internal domain/ports deps: {results:#?}"
    );
}

#[test]
fn cross_app_path_dep_is_owned_by_rule_24_not_rule_21() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_21 = assertions::errors_by_id(&results, "RS-HEXARCH-21");
    let rule_24 = assertions::errors_by_id(&results, "RS-HEXARCH-24");

    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-21").is_empty(),
        "{results:#?}"
    );
    assert_eq!(
        rule_24.len(),
        1,
        "rule 24 should own cross-app domain path deps: {rule_24:#?}"
    );
    assert!(
        rule_21.is_empty(),
        "rule 21 should stay out of cross-app ownership: {rule_21:#?}"
    );
}
