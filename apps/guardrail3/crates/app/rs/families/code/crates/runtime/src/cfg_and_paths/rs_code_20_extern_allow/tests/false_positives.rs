use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_20_extern_allow::assert_no_hits;
use test_support::write_file;

#[test]
fn ignores_allow_attrs_that_do_not_cover_extern_blocks() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let api_content = test_support::read_file(root, api_rel);

    write_file(
        root,
        api_rel,
        &format!(
            "{api_content}\n#[allow(dead_code)] // reason: test-only helper required for sidecar wiring\nfn local_probe() {{}}\n#[allow(improper_ctypes)]\nunsafe extern \"C\" fn extern_probe() {{}}\nextern \"C\" {{\n    #[allow(improper_ctypes)]\n    fn foreign_probe(code: i32);\n}}\n#[allow(improper_ctypes)]\nextern crate core;\n#[allow(improper_ctypes)]\nmod ffi_module_probe {{\n    unsafe extern \"C\" {{\n        fn inner_probe(code: i32);\n    }}\n}}\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
