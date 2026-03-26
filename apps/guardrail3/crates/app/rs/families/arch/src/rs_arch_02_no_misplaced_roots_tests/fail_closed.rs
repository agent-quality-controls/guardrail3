use super::super::super::test_support::{assert_error_files, check_results, entry, tree};

#[test]
fn malformed_guardrail_config_does_not_suppress_misplaced_root_reporting() {
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", "[rust.checks]\nhexarch = \"nope\"\n"),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
    assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
}
