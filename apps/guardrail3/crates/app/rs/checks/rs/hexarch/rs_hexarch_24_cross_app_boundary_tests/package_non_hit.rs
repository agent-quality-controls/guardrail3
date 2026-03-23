use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn packages_path_dependency_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\nshared-types = { path = \"../../../../packages/shared-types\" }\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-24");
}
