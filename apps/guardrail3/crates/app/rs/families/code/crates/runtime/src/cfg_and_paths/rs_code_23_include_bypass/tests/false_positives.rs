use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_23_include_bypass::assert_no_hits;
use test_support::write_file;

#[test]
fn ignores_non_traversing_include_str_without_rust_include_bypass() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let rest_content = test_support::read_file(root, rest_rel);

    write_file(
        root,
        rest_rel,
        &format!(
            "{rest_content}\nconst LOCAL_TEMPLATE: &str = include_str!(\"embedded_schema.json\");\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
