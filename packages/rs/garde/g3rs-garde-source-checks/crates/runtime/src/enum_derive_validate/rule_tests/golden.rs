use g3rs_garde_source_checks_assertions::enum_derive_validate::rule as assertions;

#[test]
fn stays_quiet_for_validated_enum_boundary() {
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
