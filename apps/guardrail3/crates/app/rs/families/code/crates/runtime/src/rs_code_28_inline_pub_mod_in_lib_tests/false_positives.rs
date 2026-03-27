use guardrail3_app_rs_family_code_assertions::rs_code_28_inline_pub_mod_in_lib::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_non_public_or_file_backed_modules() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        test_support::read_file(root, package_rel);

    write_file(
        root,
        package_rel,
        &format!("{package_content}\n\nmod internal {{ pub fn ping() {{}} }}\npub mod api;\n"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
