use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn attacks_allow_attrs_on_extern_blocks_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let mcp_rel = "apps/backend/crates/adapters/inbound/mcp/crates/domain/protocol/src/lib.rs";
    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";

    let mcp_content = std::fs::read_to_string(root.join(mcp_rel)).expect("read mcp source");
    let api_content = std::fs::read_to_string(root.join(api_rel)).expect("read api source");
    let worker_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

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
        .expect("mcp allow line")
        + 1;
    let api_line = api_mutated
        .lines()
        .position(|line| line.contains("#[allow(improper_ctypes_definitions)]"))
        .expect("api allow line")
        + 1;
    let worker_line = worker_mutated
        .lines()
        .position(|line| line.contains("#[cfg_attr(feature = \"ffi\", allow(improper_ctypes))]"))
        .expect("worker cfg_attr line")
        + 1;

    let results = run_family(root);
    let mut rs_code_20_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-20")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_20_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-20"),
        BTreeSet::from([
            mcp_rel.to_owned(),
            api_rel.to_owned(),
            worker_rel.to_owned()
        ])
    );
    assert_eq!(rs_code_20_results.len(), 3);
    assert_eq!(files_for_rule(&results, "RS-CODE-03"), BTreeSet::new());
    assert_eq!(files_for_rule(&results, "RS-CODE-04"), BTreeSet::new());
    assert_eq!(
        rs_code_20_results,
        vec![
            (
                Some(mcp_rel.to_owned()),
                Some(mcp_line),
                format!("{:?}", Severity::Error),
                "allow on extern block".to_owned(),
                "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression."
                    .to_owned(),
                false,
            ),
            (
                Some(api_rel.to_owned()),
                Some(api_line),
                format!("{:?}", Severity::Error),
                "allow on extern block".to_owned(),
                "`#[allow(improper_ctypes_definitions)]` on an `extern` block hides FFI risk behind a broad suppression."
                    .to_owned(),
                false,
            ),
            (
                Some(worker_rel.to_owned()),
                Some(worker_line),
                format!("{:?}", Severity::Error),
                "allow on extern block".to_owned(),
                "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression."
                    .to_owned(),
                false,
            ),
        ]
    );
}
