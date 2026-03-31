use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_topology_assertions::rs_topology_02_no_misplaced_roots as assertions;

#[test]
fn app_and_package_roots_do_not_trigger_misplaced_root_reporting() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
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

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
}

#[test]
fn excluded_fixture_and_target_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tests", "target"], &["guardrail3.toml"])),
            ("tests", entry(&["fixtures"], &[])),
            ("tests/fixtures", entry(&["worker"], &[])),
            ("tests/fixtures/worker", entry(&[], &["Cargo.toml"])),
            ("target", entry(&["debug"], &[])),
            ("target/debug", entry(&["scratch"], &[])),
            ("target/debug/scratch", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "tests/fixtures/worker/Cargo.toml",
                "[package]\nname = \"fixture\"\n",
            ),
            (
                "target/debug/scratch/Cargo.toml",
                "[package]\nname = \"scratch\"\n",
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
}

#[test]
fn declared_auxiliary_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["fuzz"], &["guardrail3.toml"])),
            ("fuzz", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "fuzz/Cargo.toml",
                "[package]\nname = \"fuzz\"\n\n[package.metadata.guardrail3]\narch_role = \"auxiliary\"\n",
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
}

#[test]
fn excluded_validation_root_does_not_treat_its_own_cargo_manifest_as_live_architecture() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree_at(
        "/tmp/repo/tests/fixtures/rust-app",
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("guardrail3.toml", config),
            ("Cargo.toml", "[package]\nname = \"fixture\"\n"),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
}

#[test]
fn app_scoped_validation_root_still_classifies_nested_crates_as_app_owned() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree_at(
        "/tmp/repo/apps/backend",
        &[
            ("", entry(&["crates"], &["Cargo.toml", "guardrail3.toml"])),
            ("crates", entry(&["domain"], &[])),
            ("crates/domain", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
            ("crates/domain/Cargo.toml", "[package]\nname = \"domain\"\n"),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
}
