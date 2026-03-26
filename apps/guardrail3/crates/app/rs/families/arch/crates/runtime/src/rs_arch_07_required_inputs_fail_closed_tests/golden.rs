use guardrail3_app_rs_family_arch_assertions::rs_arch_07_required_inputs_fail_closed as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn golden_layout_has_no_required_input_failures() {
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "guardrail3.toml",
                "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
            ),
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("packages/shared/Cargo.toml", PACKAGE_CARGO),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-07").is_empty(),
        "unexpected required-input failures: {results:#?}"
    );
}
