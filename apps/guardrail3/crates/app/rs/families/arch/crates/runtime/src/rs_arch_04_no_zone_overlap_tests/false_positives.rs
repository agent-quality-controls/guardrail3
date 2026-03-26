use guardrail3_app_rs_family_arch_assertions::rs_arch_04_no_zone_overlap as assertions;
#[allow(unused_imports)]
use super::{cargo_fixture, CargoFixture, entry, tree, tree_at};

#[test]
fn sibling_app_and_package_roots_do_not_overlap() {
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
        assertions::error_results(&results, "RS-ARCH-04").is_empty(),
        "sibling app/package roots should not overlap: {results:#?}"
    );
}

#[test]
fn ambiguous_roots_do_not_also_emit_zone_overlap_findings() {
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["packages"], &["Cargo.toml"])),
            ("apps/backend/packages", entry(&["shared"], &[])),
            ("apps/backend/packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", cargo_fixture(CargoFixture::AppWorkspace)),
            ("apps/backend/packages/shared/Cargo.toml", cargo_fixture(CargoFixture::Package)),
        ],
    ));

    assert!(
        assertions::error_results(&results, "RS-ARCH-04").is_empty(),
        "ambiguous roots belong to RS-ARCH-01/03, not RS-ARCH-04 overlap reporting: {results:#?}"
    );
}
