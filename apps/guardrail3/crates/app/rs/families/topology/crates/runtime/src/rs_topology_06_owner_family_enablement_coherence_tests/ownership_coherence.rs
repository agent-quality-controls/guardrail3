use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_06_owner_family_enablement_coherence as assertions;

#[test]
fn app_roots_error_when_effective_hexarch_enablement_is_false() {
    let config = "[rust.checks]\ntopology = true\nhexarch = false\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "apps/backend/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-06", &["apps/backend/Cargo.toml"]);
}

#[test]
fn app_scoped_hexarch_override_false_beats_global_true() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\nhexarch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["worker"], &[])),
            ("apps/backend/crates/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
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

    assertions::assert_error_files(
        &results,
        "RS-TOPOLOGY-06",
        &[
            "apps/backend/Cargo.toml",
            "apps/backend/crates/worker/Cargo.toml",
        ],
    );
}

#[test]
fn app_scoped_hexarch_override_true_beats_global_false() {
    let config = "[rust.checks]\ntopology = true\nhexarch = false\nlibarch = true\n\n[rust.apps.backend.checks]\nhexarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["worker"], &[])),
            ("apps/backend/crates/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
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

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-06");
    assertions::assert_inventory_files(
        &results,
        "RS-TOPOLOGY-06",
        &[
            "apps/backend/Cargo.toml",
            "apps/backend/crates/worker/Cargo.toml",
        ],
    );
}
