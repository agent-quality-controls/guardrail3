use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_06_owner_family_enablement_coherence as assertions;

#[test]
fn package_roots_require_arch_enablement() {
    let config = "[rust.checks]\ntopology = true\narch = false\nhexarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &["guardrail3.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "packages/shared/Cargo.toml",
                cargo_fixture(CargoFixture::Package),
            ),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-06", &["packages/shared/Cargo.toml"]);
}

#[test]
fn package_roots_are_coherent_when_arch_is_enabled() {
    let config = "[rust.checks]\ntopology = true\narch = true\nhexarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &["guardrail3.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
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
        &["packages/shared/Cargo.toml"],
    );
}
