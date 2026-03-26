use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_21_domain_purity as assertions;
use super::{copy_fixture, write_file};

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

    let results = super::run_family(tmp.path());
    assertions::assert_error_results(
        &results,
        "",
        2,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        &[
            "domain crate `backend-domain-engine` depends on disallowed external crate `vendor-domain-kit`",
            "domain crate `backend-domain-engine` depends on disallowed external crate `vendor-ports-kit`",
        ],
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

    let results = super::run_family(tmp.path());
    assertions::assert_error_results(
        &results,
        "RS-HEXARCH-24",
        1,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        &["cross-app boundary dependency"],
    );
    assertions::assert_no_error(&results, "");
}
