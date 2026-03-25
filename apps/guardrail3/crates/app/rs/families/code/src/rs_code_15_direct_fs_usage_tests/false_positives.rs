use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_test_boundaries_cfg_test_and_allowed_std_fs_usage() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let cfg_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let allowed_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";
    let test_text_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let cfg_content = std::fs::read_to_string(root.join(cfg_rel)).expect("read cfg source");
    let allowed_content =
        std::fs::read_to_string(root.join(allowed_rel)).expect("read allowed source");
    let test_text_content =
        std::fs::read_to_string(root.join(test_text_rel)).expect("read text source");

    write_file(
        root,
        "apps/backend/crates/app/queries/tests/fs_usage_tests.rs",
        "use std::fs;\n#[test]\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/test/fs_usage_test.rs",
        "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/__tests__/fs_usage.rs",
        "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/fs_usage_test.rs",
        "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/fs_usage_tests.rs",
        "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/tests.rs",
        "use std::fs;\npub fn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/fs.rs",
        "use std::fs;\npub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        cfg_rel,
        &format!(
            "{cfg_content}\n#[cfg(test)]\nuse std::fs;\n#[cfg(test)]\nmod cfg_probe {{\n    pub fn run() {{ let _ = std::fs::read_to_string(\"fixture\"); }}\n}}\n"
        ),
    );
    write_file(
        root,
        allowed_rel,
        &format!(
            "{allowed_content}\n#[allow(clippy::disallowed_methods)]\nfn allowed_free_fn() {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n#[allow(clippy::disallowed_methods)]\nmod allowed_mod {{\n    pub fn run() {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n}}\nstruct AllowProbe;\nimpl AllowProbe {{\n    #[allow(clippy::disallowed_methods)]\n    fn run(&self) {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n}}\nfn local_allow_probe() {{\n    #[allow(clippy::disallowed_methods)]\n    let _value = std::fs::read_to_string(\"jobs.txt\");\n}}\nfn path_reference_probe() {{\n    let _reader = std::fs::read_to_string;\n}}\n"
        ),
    );
    write_file(
        root,
        test_text_rel,
        &format!(
            "{test_text_content}\nfn text_probe() {{\n    let _ = \"use std::fs\";\n    let _ = \"std::fs::read_to_string\";\n    // use std::fs\n    // std::fs::read_to_string(\"fixture\")\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_15_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-15")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-15"), BTreeSet::new());
    assert!(rs_code_15_results.is_empty());
}
