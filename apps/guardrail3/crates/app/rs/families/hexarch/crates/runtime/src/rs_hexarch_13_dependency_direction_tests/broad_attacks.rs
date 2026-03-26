use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_13_dependency_direction as assertions;
use super::{copy_fixture, write_file};

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

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(
        &results,
        "",
        3,
        &[
            "apps/backend/crates/domain/engine/Cargo.toml",
            "apps/backend/crates/ports/outbound/repo/Cargo.toml",
        ],
    );
}
