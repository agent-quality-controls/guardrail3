use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_15_direct_fs_usage::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_test_boundaries_cfg_test_src_fs_and_text_only_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let cfg_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let test_text_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let cfg_content = test_support::read_file(root, cfg_rel);
    let test_text_content = test_support::read_file(root, test_text_rel);

    write_file(
        root,
        "apps/backend/crates/app/queries/tests/fs_usage_tests.rs",
        "use std::fs;\n#[test]\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
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
        "tests/top_level_fs_usage.rs",
        "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/fs.rs",
        "use std::fs;\npub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/shared/fs/src/lib.rs",
        "pub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n",
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
        test_text_rel,
        &format!(
            "{test_text_content}\nfn text_probe() {{\n    let _ = \"use std::fs\";\n    let _ = \"std::fs::read_to_string\";\n    // use std::fs\n    // std::fs::read_to_string(\"fixture\")\n}}\n"
        ),
    );

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file.as_deref(),
                Some(path)
                    if [
                        "apps/backend/crates/app/queries/tests/fs_usage_tests.rs",
                        "apps/backend/crates/app/queries/src/fs_usage_test.rs",
                        "apps/backend/crates/app/queries/src/fs_usage_tests.rs",
                        "tests/top_level_fs_usage.rs",
                        "apps/backend/crates/app/queries/src/fs.rs",
                        "apps/backend/crates/shared/fs/src/lib.rs",
                        cfg_rel,
                        test_text_rel,
                    ]
                    .contains(&path)
            )
        })
        .collect::<Vec<_>>();
    assert_no_hits(&relevant_results);
}
