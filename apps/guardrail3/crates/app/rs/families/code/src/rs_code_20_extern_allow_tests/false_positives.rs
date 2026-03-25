use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_allow_attrs_that_do_not_cover_extern_blocks() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let api_content = std::fs::read_to_string(root.join(api_rel)).expect("read api source");

    write_file(
        root,
        api_rel,
        &format!(
            "{api_content}\n#[allow(dead_code)]\nfn local_probe() {{}}\n#[allow(improper_ctypes)]\nunsafe extern \"C\" fn extern_probe() {{}}\nextern \"C\" {{\n    #[allow(improper_ctypes)]\n    fn foreign_probe(code: i32);\n}}\n#[allow(improper_ctypes)]\nextern crate core;\n#[allow(improper_ctypes)]\nmod ffi_module_probe {{\n    unsafe extern \"C\" {{\n        fn inner_probe(code: i32);\n    }}\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_20_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-20")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-20"), BTreeSet::new());
    assert!(rs_code_20_results.is_empty());
}
