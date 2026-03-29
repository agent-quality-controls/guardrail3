use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_09_file_length::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_test_file_even_when_it_exceeds_threshold() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let filler = "fn filler() {}\n".repeat(600);
    for rel in [
        "apps/backend/crates/app/commands/tests/long_case_tests.rs",
        "apps/backend/crates/app/commands/src/long_case_test.rs",
        "apps/backend/crates/app/commands/src/long_case_tests.rs",
    ] {
        write_file(root, rel, &filler);
    }

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn skips_file_with_many_comment_lines_but_only_500_effective_lines() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let effective = "fn filler() {}\n".repeat(500);
    let comments = "// comment padding\n".repeat(200);
    write_file(root, rel, &format!("{effective}{comments}"));

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn skips_nested_block_comment_lines_when_counting_effective_length() {
    let content = format!(
        "fn keep() {{}}\n{}\nfn also_keep() {{}}\n",
        "/* outer\n/* inner */\nstill outer */\n".repeat(400)
    );

    let results = super::super::check_source("src/lib.rs", &content, false);
    assert_no_hits(&results);
}

#[test]
fn skips_multiline_raw_string_payload_lines_when_counting_effective_length() {
    let content = format!(
        "const HELP: &str = r#\"\n{}\n\"#;\nfn keep() {{}}\n",
        "payload line\n".repeat(600)
    );

    let results = super::super::check_source("src/lib.rs", &content, false);
    assert_no_hits(&results);
}
