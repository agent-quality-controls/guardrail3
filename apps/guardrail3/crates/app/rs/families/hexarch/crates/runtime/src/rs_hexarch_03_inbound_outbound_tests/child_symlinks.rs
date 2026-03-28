use super::{copy_fixture, remove_dir};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn directional_child_symlink_to_valid_directory_hits_missing_for_that_container() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/adapters/outbound"),
        tmp.path().join("apps/devctl/crates/adapters/inbound"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/adapters",
        1,
        &["missing", "crates/adapters/inbound/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn nested_directional_child_symlink_to_valid_directory_hits_only_the_nested_container() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{}/ports/outbound", inner_hex()));
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{}/ports/inbound", inner_hex())),
        tmp.path().join(format!("{}/ports/outbound", inner_hex())),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        &format!("{}/ports", inner_hex()),
        1,
        &["adapters/inbound/mcp/crates/ports/outbound/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn unexpected_directional_child_symlink_is_still_reported_as_unexpected() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/adapters/outbound"),
        tmp.path().join("apps/devctl/crates/adapters/shared"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/adapters/shared",
        1,
        &["unexpected", "crates/adapters/shared/"],
        &[],
        &[],
        &[],
    );
}
