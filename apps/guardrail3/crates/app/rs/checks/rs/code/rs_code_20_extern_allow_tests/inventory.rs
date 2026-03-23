use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_allow_attrs_on_extern_blocks_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let mcp_rel = "apps/backend/crates/adapters/inbound/mcp/crates/domain/protocol/src/lib.rs";
    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";

    let mcp_content = std::fs::read_to_string(root.join(mcp_rel)).expect("read mcp source");
    let api_content = std::fs::read_to_string(root.join(api_rel)).expect("read api source");

    let mcp_line = mcp_content.lines().count() + 2;
    let api_line = api_content.lines().count() + 2;

    write_file(
        root,
        mcp_rel,
        &format!(
            "{mcp_content}\n#[allow(improper_ctypes)]\nunsafe extern \"C\" {{\n    fn protocol_probe(code: i32);\n}}\n"
        ),
    );
    write_file(
        root,
        api_rel,
        &format!(
            "{api_content}\n#[allow(improper_ctypes_definitions)]\nunsafe extern \"C\" {{\n    fn api_probe(code: i32);\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_20_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-20")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-20"),
        BTreeSet::from([mcp_rel.to_owned(), api_rel.to_owned()])
    );
    assert_eq!(rs_code_20_results.len(), 2);
    assert_eq!(
        rs_code_20_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line))
            .collect::<Vec<_>>(),
        vec![
            (Some(mcp_rel), Some(mcp_line)),
            (Some(api_rel), Some(api_line)),
        ]
    );
}
