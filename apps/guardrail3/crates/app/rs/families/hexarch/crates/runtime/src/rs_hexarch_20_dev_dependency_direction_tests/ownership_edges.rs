use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_20_dev_dependency_direction as assertions;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_17_workspace_inherited_direction as rule17_assertions;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_18_renamed_dependency_direction as rule18_assertions;
use super::{copy_fixture, write_file};

#[test]
fn renamed_dev_edge_is_owned_by_rule_20_not_rule_18() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_count(&results, "", 1);
    rule18_assertions::assert_no_error(&results, "");
}

#[test]
fn inherited_dev_edge_is_owned_by_rule_20_not_rule_17() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n  \"crates/app/*\",\n  \"crates/domain/*\",\n  \"crates/ports/inbound/*\",\n  \"crates/ports/outbound/*\",\n  \"crates/adapters/inbound/*\",\n  \"crates/adapters/outbound/*\",\n]\n[workspace.dependencies]\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"crates/adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\n[dev-dependencies]\nqueue_alias = { workspace = true }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_count(&results, "", 1);
    rule17_assertions::assert_no_error(&results, "");
}
