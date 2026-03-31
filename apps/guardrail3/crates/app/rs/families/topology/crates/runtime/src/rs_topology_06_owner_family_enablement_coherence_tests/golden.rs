use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_06_owner_family_enablement_coherence as assertions;

#[test]
fn golden_layout_has_no_owner_family_coherence_errors() {
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

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-06");
    assertions::assert_inventory_files(
        &results,
        "RS-TOPOLOGY-06",
        &["apps/backend/Cargo.toml", "packages/shared/Cargo.toml"],
    );
}
