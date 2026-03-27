use guardrail3_app_rs_family_code_assertions::rs_code_14_unwrap_expect::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_cfg_test_and_text_only_unwrap_expect_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let cfg_test_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let text_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let cfg_test_content =
        test_support::read_file(root, cfg_test_rel);
    let text_content = test_support::read_file(root, text_rel);

    write_file(
        root,
        cfg_test_rel,
        &format!(
            "{cfg_test_content}\n#[cfg(test)]\nfn cfg_test_probe() {{ let _ = Some(1).unwrap(); }}\n#[cfg(test)]\nfn cfg_test_expect_probe() {{ let _ = Some(1).expect(\"ok\"); }}\n"
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
    let relevant_results = results
        .into_iter()
        .filter(|result| matches!(result.file.as_deref(), Some(path) if [cfg_test_rel, text_rel].contains(&path)))
        .collect::<Vec<_>>();
    assert_no_hits(&relevant_results);
}
