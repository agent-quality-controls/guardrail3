#[allow(unused_imports)]
use super::{CargoFixture, cargo_fixture, check_results, entry, tree, tree_at};
use guardrail3_app_rs_family_arch_assertions::rs_arch_04_no_zone_overlap as assertions;

#[test]
fn nested_cross_zone_roots_do_not_emit_overlap_on_top_of_ambiguity_and_dual_ownership() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["packages"], &["Cargo.toml"])),
            ("apps/backend/packages", entry(&["shared"], &[])),
            ("apps/backend/packages/shared", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["core"], &[])),
            ("packages/core", entry(&["apps"], &["Cargo.toml"])),
            ("packages/core/apps", entry(&["web"], &[])),
            ("packages/core/apps/web", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
            (
                "apps/backend/packages/shared/Cargo.toml",
                cargo_fixture(CargoFixture::Package),
            ),
            (
                "packages/core/Cargo.toml",
                cargo_fixture(CargoFixture::Package),
            ),
            (
                "packages/core/apps/web/Cargo.toml",
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-ARCH-04");
}
