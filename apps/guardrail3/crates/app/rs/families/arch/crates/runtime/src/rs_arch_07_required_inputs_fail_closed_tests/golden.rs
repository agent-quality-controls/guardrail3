#[allow(unused_imports)]
use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_arch_assertions::rs_arch_07_required_inputs_fail_closed as assertions;

#[test]
fn golden_layout_has_no_required_input_failures() {
    let results = check_results(&tree(
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

    assert!(
        assertions::error_results(&results, "RS-ARCH-07").is_empty(),
        "unexpected required-input failures: {results:#?}"
    );
}
