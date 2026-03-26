#[allow(unused_imports)]
use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_arch_assertions::rs_arch_01_root_classification as assertions;

#[test]
fn golden_layout_has_no_classification_errors() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
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
        assertions::error_results(&results, "RS-ARCH-01").is_empty(),
        "unexpected classification errors: {results:#?}"
    );
}
