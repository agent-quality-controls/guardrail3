use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_18_renamed_dependency_direction as assertions;

#[test]
fn forbidden_renamed_edges_error_and_unrenamed_edges_do_not() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nengine_types = { package = \"backend-domain-types\", path = \"../types\" }\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/Cargo.toml",
        "[package]\nname = \"backend-ports-outbound-repo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nrepo_types = { package = \"backend-domain-types\", path = \"../../../domain/types\" }\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(
        &results,
        "",
        2,
        &[
            "apps/backend/crates/domain/engine/Cargo.toml",
            "apps/backend/crates/ports/outbound/repo/Cargo.toml",
        ],
    );
}

#[test]
fn messages_name_both_alias_and_package_for_each_forbidden_edge() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\ncommands_alias = { package = \"backend-app-commands\", path = \"../../app/commands\" }\nqueue_alias = { package = \"backend-adapters-outbound-queue\", path = \"../../adapters/outbound/queue\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 2);
    assertions::assert_error_messages_contain(
        &results,
        "",
        &[
            &["alias `commands_alias`", "package `backend-app-commands`"],
            &[
                "alias `queue_alias`",
                "package `backend-adapters-outbound-queue`",
            ],
        ],
    );
}
