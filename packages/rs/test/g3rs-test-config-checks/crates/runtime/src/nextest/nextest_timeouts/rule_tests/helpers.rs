use cargo_toml_parser::parse as parse_cargo;
use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::G3CheckResult;
use nextest_toml_parser::parse as parse_nextest;

pub(super) fn input() -> G3RsTestConfigChecksInput {
    G3RsTestConfigChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        mutants_rel_path: ".cargo/mutants.toml".to_owned(),
        nextest_rel_path: ".config/nextest.toml".to_owned(),
        cargo: parse_cargo("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n")
            .expect("valid Cargo.toml fixture"),
        nextest: None,
        mutants: None,
        has_tests: false,
        has_tokio_tests: false,
        tokio_dependency_present: false,
        cargo_mutants_installed: false,
        mutation_hook_active: false,
        mutation_hook_files: Vec::new(),
        mutants_exists: false,
    }
}

pub(super) fn with_nextest(
    mut input: G3RsTestConfigChecksInput,
    content: &str,
) -> G3RsTestConfigChecksInput {
    input.nextest = Some(parse_nextest(content).expect("valid nextest fixture"));
    input
}

pub(super) fn run(input: &G3RsTestConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    super::super::check(input, &mut results);
    results
}
