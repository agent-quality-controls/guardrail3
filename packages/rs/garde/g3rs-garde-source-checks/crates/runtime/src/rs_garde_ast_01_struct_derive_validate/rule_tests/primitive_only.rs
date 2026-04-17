use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate::rule as assertions;

#[test]
fn stays_quiet_for_primitive_only_boundary_without_validate() {
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use clap::Parser;\n\n#[derive(Parser)]\nstruct Input {\n    enabled: bool,\n    count: usize,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
