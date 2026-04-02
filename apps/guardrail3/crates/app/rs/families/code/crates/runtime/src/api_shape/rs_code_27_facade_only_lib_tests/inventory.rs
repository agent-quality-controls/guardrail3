use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_27_facade_only_lib::{
    RuleFinding, Severity, assert_files, assert_findings,
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
    let use_line = mutated
        .lines()
        .position(|line| line.contains("use crate::TenantSlug;"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let fn_line = mutated
        .lines()
        .position(|line| line.contains("pub fn mutate_surface()"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let inline_line = mutated
        .lines()
        .position(|line| line.contains("pub mod api"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([package_rel.to_owned()]));
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Error,
                "lib.rs should stay facade-only",
                "lib.rs contains private use `crate::TenantSlug`. Keep lib.rs limited to facade declarations and type/const definitions.",
                Some(package_rel),
                Some(use_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "lib.rs should stay facade-only",
                "lib.rs contains function `mutate_surface`. Keep lib.rs limited to facade declarations and type/const definitions.",
                Some(package_rel),
                Some(fn_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "lib.rs should stay facade-only",
                "lib.rs contains inline module `api`. Keep lib.rs limited to facade declarations and type/const definitions.",
                Some(package_rel),
                Some(inline_line),
                false,
            ),
        ],
    );
}
