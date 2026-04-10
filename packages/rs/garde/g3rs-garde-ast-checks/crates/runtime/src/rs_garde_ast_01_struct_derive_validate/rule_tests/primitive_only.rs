use g3rs_garde_ast_checks_assertions::rs_garde_ast_01_struct_derive_validate as assertions;

#[test]
fn stays_quiet_for_primitive_only_boundary_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use clap::Parser;\n\n#[derive(Parser)]\nstruct Input {\n    enabled: bool,\n    count: usize,\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
