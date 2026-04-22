use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate::rule as assertions;

#[test]
fn stays_quiet_for_validated_struct_boundary() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "Input",
        &["Deserialize"],
        true,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
