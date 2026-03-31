use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_07_required_inputs_fail_closed as assertions;

#[test]
fn golden_layout_has_no_required_input_failures() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "guardrail3.toml",
                "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
            ),
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

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-07");
    assertions::assert_inventory_summary(
        &results,
        "RS-TOPOLOGY-07",
        "Rust architecture required inputs are readable",
    );
}
