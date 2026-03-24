use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn external_same_name_collision_does_not_count_as_cross_app_path_dep() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\nworker-domain-jobs = \"1\"\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-24");
}

#[test]
fn broken_cross_app_path_does_not_count_as_boundary_violation() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nworker-domain-missing = { path = \"../../../../worker/crates/domain/missing\" }\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-24");
}
