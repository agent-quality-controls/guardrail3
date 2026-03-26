use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, PACKAGE_CARGO, check_results, entry, error_results, tree,
};

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
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("packages/shared/Cargo.toml", PACKAGE_CARGO),
        ],
    ));

    assert!(
        error_results(&results, "RS-ARCH-02").is_empty(),
        "valid zone-owned roots should not be reported as misplaced: {results:#?}"
    );
}

#[test]
fn excluded_fixture_and_target_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
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

    assert!(
        error_results(&results, "RS-ARCH-02").is_empty(),
        "excluded fixture/target manifests must not be reported as live misplaced roots: {results:#?}"
    );
}

#[test]
fn declared_auxiliary_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
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

    assert!(
        error_results(&results, "RS-ARCH-02").is_empty(),
        "declared auxiliary roots must not be reported as misplaced: {results:#?}"
    );
}
