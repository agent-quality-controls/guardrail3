use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_18_renamed_dependency_direction as assertions;
use super::{copy_fixture, write_file};

#[test]
fn cross_app_renamed_edge_is_owned_by_rule_24_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nprocessor_alias = { package = \"worker-app-processor\", path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_results(
        &results,
        "RS-HEXARCH-24",
        1,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        &["cross-app boundary dependency"],
    );
    assertions::assert_error_count(&results, "", 0);
}

#[test]
fn inherited_renamed_path_dep_is_owned_by_rule_17_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n  \"crates/app/*\",\n  \"crates/domain/*\",\n  \"crates/ports/inbound/*\",\n  \"crates/ports/outbound/*\",\n  \"crates/adapters/inbound/*\",\n  \"crates/adapters/outbound/*\",\n]\n[workspace.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"crates/adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nqueue_alias = { workspace = true }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "RS-HEXARCH-17", 1);
    assertions::assert_error_count(&results, "", 0);
}

#[test]
fn renamed_dev_dependency_is_owned_by_rule_20_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_count(&results, "RS-HEXARCH-20", 1);
    assertions::assert_error_count(&results, "", 0);
}

#[test]
fn renamed_target_dependency_is_owned_by_rule_25_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[target.'cfg(unix)'.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_results(
        &results,
        "RS-HEXARCH-25",
        1,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        &["target dependency direction violation"],
    );
    assertions::assert_error_count(&results, "", 0);
}
