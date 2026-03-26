use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_24_cross_app_boundary as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn packages_path_dependency_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\nshared-types = { path = \"../../../../packages/shared-types\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-24").is_empty(),
        "{results:#?}"
    );
}
