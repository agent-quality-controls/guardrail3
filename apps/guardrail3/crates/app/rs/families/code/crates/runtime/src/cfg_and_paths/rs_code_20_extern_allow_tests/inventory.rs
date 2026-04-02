use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_20_extern_allow::assert_inventory_allow_attrs_on_extern_blocks;
use test_support::write_file;

#[test]
fn attacks_allow_attrs_on_extern_blocks_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let mcp_rel = "apps/backend/crates/adapters/inbound/mcp/crates/domain/protocol/src/lib.rs";
    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";

    let mcp_content = test_support::read_file(root, mcp_rel);
    let api_content = test_support::read_file(root, api_rel);
    let worker_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let worker_content = test_support::read_file(root, worker_rel);

    let mcp_mutated = format!(
        "{mcp_content}\nmod native_protocol {{\n    #[allow(improper_ctypes)]\n    unsafe extern \"C\" {{\n        fn protocol_probe(code: i32);\n    }}\n}}\n"
    );
    let api_mutated = format!(
        "{api_content}\n#[allow(improper_ctypes_definitions)] // reason: ffi compatibility shim\nunsafe extern \"C\" {{\n    fn api_probe(code: i32);\n}}\n"
    );
    let worker_mutated = format!(
        "{worker_content}\n#[cfg_attr(feature = \"ffi\", allow(improper_ctypes))]\nunsafe extern \"C\" {{\n    fn worker_probe(code: i32);\n}}\n"
    );

    write_file(root, mcp_rel, &mcp_mutated);
    write_file(root, api_rel, &api_mutated);
    write_file(root, worker_rel, &worker_mutated);

    let mcp_line = mcp_mutated
        .lines()
        .position(|line| line.contains("#[allow(improper_ctypes)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let api_line = api_mutated
        .lines()
        .position(|line| line.contains("#[allow(improper_ctypes_definitions)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_line = worker_mutated
        .lines()
        .position(|line| line.contains("#[cfg_attr(feature = \"ffi\", allow(improper_ctypes))]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_inventory_allow_attrs_on_extern_blocks(
        &run_family(root),
        [mcp_rel, api_rel, worker_rel],
        [mcp_line, api_line, worker_line],
    );
}
