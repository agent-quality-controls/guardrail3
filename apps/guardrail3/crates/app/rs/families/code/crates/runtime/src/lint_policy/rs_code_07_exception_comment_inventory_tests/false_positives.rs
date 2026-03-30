use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_07_exception_comment_inventory::assert_no_hits;
use test_support::write_file;

#[test]
fn ignores_exception_like_text_outside_supported_config_comment_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_package_rel = "package.json";
    let backend_cargo_rel = "apps/backend/Cargo.toml";
    let root_guardrail_rel = "guardrail3.toml";

    let root_package = test_support::read_file(root, root_package_rel);
    let backend_cargo = test_support::read_file(root, backend_cargo_rel);
    let root_guardrail = test_support::read_file(root, root_guardrail_rel);

    write_file(
        root,
        root_package_rel,
        &format!("{root_package}\n// EXCEPTION: package metadata note\n"),
    );
    write_file(
        root,
        backend_cargo_rel,
        &format!("{backend_cargo}\n# exception backend note without required marker\n"),
    );
    write_file(
        root,
        root_guardrail_rel,
        &format!("{root_guardrail}\nnote = \"# EXCEPTION: literal text only\"\n"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
