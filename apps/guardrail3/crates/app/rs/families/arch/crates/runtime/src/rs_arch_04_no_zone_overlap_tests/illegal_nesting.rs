use guardrail3_app_rs_family_arch_assertions::rs_arch_04_no_zone_overlap as assertions;
#[allow(unused_imports)]
use super::{cargo_fixture, CargoFixture, entry, tree, tree_at};

#[test]
fn nested_cross_zone_roots_do_not_emit_overlap_on_top_of_ambiguity_and_dual_ownership() {
    let results = assertions::check_results(&tree(
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
            ("apps/backend/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
            ("apps/backend/packages/shared/Cargo.toml", cargo_fixture(CargoFixture::Package)),
            ("packages/core/Cargo.toml", cargo_fixture(CargoFixture::Package)),
            ("packages/core/apps/web/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-04").is_empty(),
        "cross-zone nested roots should be owned by RS-ARCH-01 and RS-ARCH-03, not also RS-ARCH-04: {results:#?}"
    );
}
