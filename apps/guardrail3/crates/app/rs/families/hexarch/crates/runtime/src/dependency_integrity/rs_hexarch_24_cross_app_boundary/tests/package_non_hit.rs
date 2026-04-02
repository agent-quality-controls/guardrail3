use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_24_cross_app_boundary as assertions;

#[test]
fn packages_path_dependency_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\nshared-types = { path = \"../../../../packages/shared-types\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
