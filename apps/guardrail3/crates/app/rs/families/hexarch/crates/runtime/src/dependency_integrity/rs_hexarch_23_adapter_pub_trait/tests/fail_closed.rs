use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_23_adapter_pub_trait as assertions;

#[test]
fn unparsable_adapter_source_errors_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/lib.rs"),
        "pub trait Broken {\n",
    )
    .expect("failed to write broken adapter source into hexarch fixture");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_single(
        &results,
        "",
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
    );
    assertions::assert_error_message_contains(&results, "", &["Failed to parse Rust source file"]);
}

#[test]
fn parse_failure_takes_precedence_over_public_trait_violation() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/lib.rs"),
        "mod extra;\npub trait Broken {\n",
    )
    .expect("failed to write broken adapter source into hexarch fixture");
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/extra.rs"),
        "pub trait ExtraBoundary {}\n",
    )
    .expect("write public-trait extra module");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_single(
        &results,
        "",
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
    );
    assertions::assert_error_message_contains(&results, "", &["Failed to parse Rust source file"]);
    assertions::assert_error_title_forbidden(&results, "", &["defines public traits"]);
}
