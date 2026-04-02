use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_18_renamed_dependency_direction as assertions;

#[test]
fn renamed_external_dependency_without_internal_resolution_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[dependencies]\nbackend_ports_outbound_repo = { package = \"tokio\", version = \"1\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn renamed_missing_same_app_layer_like_path_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/missing\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn renamed_allowed_same_app_edge_stays_clean() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/commands/Cargo.toml",
        "[package]\nname = \"backend-app-commands\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nengine_alias = { package = \"backend-domain-engine\", path = \"../../domain/engine\" }\ntypes_alias = { package = \"backend-domain-types\", path = \"../../domain/types\" }\nrepo_alias = { package = \"backend-ports-outbound-repo\", path = \"../../ports/outbound/repo\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn renamed_existing_same_app_target_omitted_from_workspace_still_fires() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n  \"crates/app/*\",\n  \"crates/domain/*\",\n  \"crates/ports/inbound/*\",\n  \"crates/ports/outbound/*\",\n  \"crates/adapters/inbound/*\",\n]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 1);
}
