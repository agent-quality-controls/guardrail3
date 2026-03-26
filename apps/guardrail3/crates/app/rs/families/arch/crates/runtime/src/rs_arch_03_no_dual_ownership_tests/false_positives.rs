use guardrail3_app_rs_family_arch_assertions::rs_arch_03_no_dual_ownership as assertions;
#[allow(unused_imports)]
use super::{cargo_fixture, CargoFixture, entry, tree, tree_at};

#[test]
fn nested_app_inside_app_does_not_count_as_dual_family_ownership() {
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["apps"], &["Cargo.toml"])),
            ("apps/backend/apps", entry(&["worker"], &[])),
            ("apps/backend/apps/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
            ("apps/backend/apps/worker/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-03").is_empty(),
        "multiple app candidates belong to RS-ARCH-01, not RS-ARCH-03: {results:#?}"
    );
}
