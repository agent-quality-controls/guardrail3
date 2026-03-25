use super::super::super::test_support::{
    assert_error_files, check_results, entry, error_results, tree,
};

#[test]
fn misplaced_roots_fire_when_hexarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_fire_when_libarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_do_not_fire_when_both_architecture_families_are_disabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assert!(
        error_results(&results, "RS-ARCH-02").is_empty(),
        "misplaced-root reporting should shut off only when both owners are disabled: {results:#?}"
    );
}
