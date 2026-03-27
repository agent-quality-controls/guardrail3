use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_cfg_test_and_allow_scoped_unwrap_expect_usage() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let cfg_test_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let impl_allow_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let local_allow_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let text_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let cfg_test_content =
        std::fs::read_to_string(root.join(cfg_test_rel)).expect("read cfg test source");
    let impl_allow_content =
        std::fs::read_to_string(root.join(impl_allow_rel)).expect("read impl source");
    let local_allow_content =
        std::fs::read_to_string(root.join(local_allow_rel)).expect("read local source");
    let text_content = std::fs::read_to_string(root.join(text_rel)).expect("read text source");

    write_file(
        root,
        cfg_test_rel,
        &format!(
            "{cfg_test_content}\n#[cfg(test)]\nfn cfg_test_probe() {{ let _ = Some(1).unwrap(); }}\n#[cfg(test)]\nfn cfg_test_expect_probe() {{ let _ = Some(1).expect(\"ok\"); }}\n#[allow(clippy::unwrap_used)]\nfn allowed_unwrap_probe() {{ let _ = Some(1).unwrap(); }}\n#[allow(clippy::expect_used)]\nfn allowed_expect_probe() {{ let _ = Some(1).expect(\"allowed\"); }}\n#[allow(clippy::unwrap_used)]\nmod allowed_module_probe {{\n    pub fn unwrap_inside_module() {{ let _ = Some(1).unwrap(); }}\n}}\n#[allow(clippy::expect_used)]\nmod allowed_expect_module_probe {{\n    pub fn expect_inside_module() {{ let _ = Some(1).expect(\"module\"); }}\n}}\n"
        ),
    );
    write_file(
        root,
        impl_allow_rel,
        &format!(
            "{impl_allow_content}\nstruct AllowProbe;\nimpl AllowProbe {{\n    #[allow(clippy::unwrap_used)]\n    fn allowed_unwrap(&self) {{ let _ = Some(1).unwrap(); }}\n    #[allow(clippy::expect_used)]\n    fn allowed_expect(&self) {{ let _ = Some(1).expect(\"allowed\"); }}\n}}\n"
        ),
    );
    write_file(
        root,
        local_allow_rel,
        &format!(
            "{local_allow_content}\nfn local_allow_probe() {{\n    #[allow(clippy::unwrap_used)]\n    let _value = Some(1).unwrap();\n    #[allow(clippy::expect_used)]\n    let _other = Some(1).expect(\"local\");\n}}\n"
        ),
    );
    write_file(
        root,
        text_rel,
        &format!(
            "{text_content}\nfn text_probe() {{\n    let _ = \".unwrap() in string\";\n    let _ = \".expect() in string\";\n    // .unwrap() in comment\n    // .expect() in comment\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_14_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-14")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-14"), BTreeSet::new());
    assert!(rs_code_14_results.is_empty());
}
