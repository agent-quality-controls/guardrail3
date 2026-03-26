use guardrail3_app_rs_family_arch_assertions::rs_arch_05_scoped_arch_config_forbidden as assertions;
#[allow(unused_imports)]
use super::{cargo_fixture, CargoFixture, entry, tree, tree_at};

#[test]
fn golden_layout_has_no_scoped_arch_errors() {
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
            ("packages/shared/Cargo.toml", cargo_fixture(CargoFixture::Package)),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-05").is_empty(),
        "unexpected scoped arch errors: {results:#?}"
    );
}
