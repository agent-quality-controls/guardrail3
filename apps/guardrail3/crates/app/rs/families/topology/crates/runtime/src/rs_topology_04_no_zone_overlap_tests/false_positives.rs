use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_04_no_zone_overlap as assertions;

#[test]
fn sibling_app_and_package_roots_do_not_overlap() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
            (
                "packages/shared/Cargo.toml",
                cargo_fixture(CargoFixture::Package),
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-04");
}

#[test]
fn same_zone_nesting_does_not_emit_zone_overlap_findings() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["worker"], &[])),
            ("apps/backend/crates/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
            (
                "apps/backend/crates/worker/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-04");
}
