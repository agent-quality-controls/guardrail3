#[allow(unused_imports)]
use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_arch_assertions::rs_arch_03_no_dual_ownership as assertions;

#[test]
fn nested_app_inside_app_does_not_count_as_dual_family_ownership() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["apps"], &["Cargo.toml"])),
            ("apps/backend/apps", entry(&["worker"], &[])),
            ("apps/backend/apps/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
            (
                "apps/backend/apps/worker/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-ARCH-03");
}
