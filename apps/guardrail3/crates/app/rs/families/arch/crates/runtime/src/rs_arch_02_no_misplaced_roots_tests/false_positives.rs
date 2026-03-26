use guardrail3_app_rs_family_arch_assertions::rs_arch_02_no_misplaced_roots as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn app_and_package_roots_do_not_trigger_misplaced_root_reporting() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "valid zone-owned roots should not be reported as misplaced: {results:#?}"
    );
}

#[test]
fn excluded_fixture_and_target_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "excluded fixture/target manifests must not be reported as live misplaced roots: {results:#?}"
    );
}

#[test]
fn declared_auxiliary_roots_do_not_trigger_misplaced_reporting() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "declared auxiliary roots must not be reported as misplaced: {results:#?}"
    );
}

#[test]
fn excluded_validation_root_does_not_treat_its_own_cargo_manifest_as_live_architecture() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree_at(
        "/tmp/repo/tests/fixtures/rust-app",
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("guardrail3.toml", config),
            ("Cargo.toml", "[package]\nname = \"fixture\"\n"),
        ],
    ));

    assert!(
        results.is_empty(),
        "an excluded validation root must not emit live arch findings for its own Cargo root: {results:#?}"
    );
}

#[test]
fn app_scoped_validation_root_still_classifies_nested_crates_as_app_owned() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree_at(
        "/tmp/repo/apps/backend",
        &[
            ("", entry(&["crates"], &["Cargo.toml", "guardrail3.toml"])),
            ("crates", entry(&["domain"], &[])),
            ("crates/domain", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("Cargo.toml", APP_WORKSPACE_CARGO),
            ("crates/domain/Cargo.toml", "[package]\nname = \"domain\"\n"),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "nested crates under an apps/<name> validation root must still classify as app-owned: {results:#?}"
    );
}
