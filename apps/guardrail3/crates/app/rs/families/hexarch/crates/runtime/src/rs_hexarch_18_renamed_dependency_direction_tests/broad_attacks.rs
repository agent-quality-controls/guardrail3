use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_18_renamed_dependency_direction as assertions;
use test_support::{copy_fixture, write_file};

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-18");

    let actual_files = errors
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/backend/crates/domain/engine/Cargo.toml".to_owned(),
        "apps/backend/crates/ports/outbound/repo/Cargo.toml".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected renamed-direction hit set: {errors:#?}"
    );
    assert_eq!(
        errors.len(),
        2,
        "expected exactly two renamed-direction errors: {errors:#?}"
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-18");
    assert_eq!(
        errors.len(),
        2,
        "expected both forbidden renamed edges from one manifest to be reported: {errors:#?}"
    );

    let messages = errors
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();
    assert!(
        messages.iter().any(|message| {
            message.contains("alias `commands_alias`")
                && message.contains("package `backend-app-commands`")
        }),
        "expected one message to name the app alias and package: {messages:#?}"
    );
    assert!(
        messages.iter().any(|message| {
            message.contains("alias `queue_alias`")
                && message.contains("package `backend-adapters-outbound-queue`")
        }),
        "expected one message to name the adapter alias and package: {messages:#?}"
    );
}
