use guardrail3_app_rs_family_arch_assertions::rs_arch_02_no_misplaced_roots as assertions;
#[allow(unused_imports)]
use super::{cargo_fixture, CargoFixture, entry, tree, tree_at};

#[test]
fn misplaced_roots_fire_when_hexarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = false\n";
    let results = assertions::check_results(&tree(
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

    assertions::assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_fire_when_libarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = true\n";
    let results = assertions::check_results(&tree(
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

    assertions::assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_do_not_fire_when_both_architecture_families_are_disabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = false\n";
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "misplaced-root reporting should shut off only when both owners are disabled: {results:#?}"
    );
}

#[test]
fn misplaced_roots_do_not_fire_when_arch_is_globally_disabled() {
    let config = "[rust.checks]\narch = false\nhexarch = true\nlibarch = true\n";
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-02").is_empty(),
        "global arch disablement must suppress misplaced-root reporting even if owner families are enabled: {results:#?}"
    );
}
