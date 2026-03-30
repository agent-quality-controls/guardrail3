use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_13_dependency_direction as assertions;

#[test]
fn omitted_same_app_target_still_hits_rule_13() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/domain/engine\",\n    \"crates/app/queries\",\n    \"crates/ports/inbound/api\",\n    \"crates/ports/outbound/repo\",\n    \"crates/ports/outbound/events\",\n    \"crates/adapters/inbound/rest\",\n    \"crates/adapters/inbound/mcp/crates/domain/protocol\",\n    \"crates/adapters/inbound/mcp/crates/app/handlers\",\n    \"crates/adapters/inbound/mcp/crates/adapters/inbound/transport\",\n    \"crates/adapters/outbound/postgres\",\n    \"crates/adapters/outbound/queue\"\n]\nresolver = \"2\"\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nbackend-app-commands = { path = \"../../app/commands\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 1);
    assertions::assert_error_file_single(
        &results,
        "",
        "apps/backend/crates/domain/engine/Cargo.toml",
    );
}

#[test]
fn renamed_same_app_edge_is_owned_by_rule_18_not_rule_13() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\ncommands_core = { package = \"backend-app-commands\", path = \"../../app/commands\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-18", 1);
}
