use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn forbidden_same_app_normal_edges_error_and_allowed_edges_do_not() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nbackend-app-commands = { path = \"../../app/commands\" }\nbackend-adapters-outbound-queue = { path = \"../../adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/Cargo.toml",
        "[package]\nname = \"backend-ports-outbound-repo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../../domain/types\" }\nbackend-adapters-outbound-postgres = { path = \"../../../adapters/outbound/postgres\" }\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-13");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/backend/crates/domain/engine/Cargo.toml".to_owned(),
        "apps/backend/crates/ports/outbound/repo/Cargo.toml".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected direction-violation hit set: {errors:#?}"
    );
    assert_eq!(
        errors.len(),
        3,
        "expected one result per forbidden edge, not just per file: {errors:#?}"
    );
}
