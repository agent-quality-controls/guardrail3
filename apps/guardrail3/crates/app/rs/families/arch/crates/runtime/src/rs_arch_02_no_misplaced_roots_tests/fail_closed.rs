use guardrail3_app_rs_family_arch_assertions::rs_arch_02_no_misplaced_roots as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn malformed_guardrail_config_does_not_suppress_misplaced_root_reporting() {
    let results = assertions::check_results(&tree(
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

    assertions::assert_error_files(&results, "RS-ARCH-02", &["tools/worker/Cargo.toml"]);
    assertions::assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
}
