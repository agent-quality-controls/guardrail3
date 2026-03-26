#[allow(unused_imports)]
use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_arch_assertions::rs_arch_01_root_classification as assertions;

#[test]
fn misplaced_other_roots_do_not_count_as_ambiguous_classification() {
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &[])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n")],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-01").is_empty(),
        "other roots should stay owned by RS-ARCH-02, not RS-ARCH-01: {results:#?}"
    );
}

#[test]
fn fixture_and_snapshot_manifests_are_not_classified_as_live_architecture() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "tests"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("tests", entry(&["fixtures", "snapshots"], &[])),
            ("tests/fixtures", entry(&["rust-app"], &[])),
            ("tests/fixtures/rust-app", entry(&[], &["Cargo.toml"])),
            ("tests/snapshots", entry(&["rust-app"], &[])),
            ("tests/snapshots/rust-app", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            (
                "tests/fixtures/rust-app/Cargo.toml",
                "[package]\nname = \"fixture\"\n",
            ),
            (
                "tests/snapshots/rust-app/Cargo.toml",
                "[package]\nname = \"snapshot\"\n",
            ),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-01").is_empty(),
        "excluded fixture/snapshot manifests must not participate in live root classification: {results:#?}"
    );
}
