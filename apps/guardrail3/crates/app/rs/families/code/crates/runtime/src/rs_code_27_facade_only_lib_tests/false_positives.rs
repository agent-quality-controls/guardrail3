use guardrail3_app_rs_family_code_assertions::rs_code_27_facade_only_lib::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_consts_types_and_explicit_pub_reexports_in_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        test_support::read_file(root, package_rel);

    write_file(
        root,
        package_rel,
        &format!(
            "{package_content}\n\npub const API_VERSION: &str = \"v1\";\npub struct FacadeMarker;\npub mod internal {{ pub struct Visible; }}\npub use internal::Visible;\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
