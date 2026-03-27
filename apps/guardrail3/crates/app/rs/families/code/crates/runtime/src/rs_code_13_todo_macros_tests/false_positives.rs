use guardrail3_app_rs_family_code_assertions::rs_code_13_todo_macros::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_unreachable_in_test_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/commands/tests/todo_macro_tests.rs";
    write_file(
        root,
        rel,
        "fn probe() {\n    let _ = \"todo! in string\";\n    // unimplemented! in comment\n    unreachable!();\n    maybe_todo();\n}\n",
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
