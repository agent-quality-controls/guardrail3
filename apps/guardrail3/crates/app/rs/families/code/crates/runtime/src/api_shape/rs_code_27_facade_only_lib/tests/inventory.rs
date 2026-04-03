use std::collections::BTreeSet;

use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_27_facade_only_lib::{
    assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn errors_on_non_facade_items_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content = test_support::read_file(root, package_rel);

    let mutated = format!(
        "{package_content}\n\nuse crate::TenantSlug;\npub fn mutate_surface() {{}}\npub mod api {{ pub fn ping() {{}} }}\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);

    // RS-CODE-27 retired: redundant with RS-ARCH-02.
    assert_files(&results, BTreeSet::new());
    assert_findings(&results, &[]);
}
