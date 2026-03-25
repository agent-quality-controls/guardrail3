use std::collections::BTreeSet;

use super::super::super::test_support::{
    assert_no_error, copy_fixture, errors_by_id, run_family, write_file,
};

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

    let results = run_family(tmp.path());
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

    let results = run_family(tmp.path());
    let rule_21 = errors_by_id(&results, "RS-HEXARCH-21");
    let rule_24 = errors_by_id(&results, "RS-HEXARCH-24");

    assert_no_error(&results, "RS-HEXARCH-21");
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
