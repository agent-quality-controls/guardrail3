use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_20_dev_dependency_direction as assertions;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_24_cross_app_boundary as rule24_assertions;
use super::{copy_fixture, write_file};

#[test]
fn out_of_tree_paths_with_layer_like_names_do_not_trigger_dev_direction() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[dev-dependencies]\nadapter-fixture = { path = \"../../../../../fixtures/adapters/http\" }\n",
    );
    write_file(
        tmp.path(),
        "fixtures/adapters/http/Cargo.toml",
        "[package]\nname = \"adapter-fixture\"\nversion = \"0.1.0\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn missing_same_app_layer_like_dev_path_does_not_trigger_dev_direction() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[dev-dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/missing\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn cross_app_dev_edge_is_owned_by_rule_24_not_rule_20() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nprocessor_alias = { package = \"worker-app-processor\", path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = super::run_family(tmp.path());
    rule24_assertions::assert_error_results(
        &results,
        "",
        1,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        &["cross-app boundary dependency"],
    );
    assertions::assert_error_count(&results, "", 0);
}
