use g3rs_code_source_checks_assertions::public_struct_named_fields::rule::assert_rule_results;

#[test]
fn skips_private_fields_tuple_structs_and_private_modules() {
    assert_rule_results(
        &super::super::check_source(
            "src/lib.rs",
            "pub struct User { id: String, email: String }",
            false,
        ),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("src/lib.rs", "pub struct UserId(pub String);", false),
        &[],
    );
    assert_rule_results(
        &super::super::check_source(
            "src/lib.rs",
            "mod internal { pub struct User { pub id: String } }",
            false,
        ),
        &[],
    );
}
