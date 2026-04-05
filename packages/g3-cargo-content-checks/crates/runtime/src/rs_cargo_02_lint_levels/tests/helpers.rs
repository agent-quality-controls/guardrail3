use cargo_toml_parser::parse as parse_cargo_toml;
use guardrail3_check_types::G3CheckResult;

use crate::rs_cargo_02_lint_levels::check;

pub(super) fn run_check(cargo_toml: &str) -> Vec<G3CheckResult> {
    let cargo = parse_cargo_toml(cargo_toml).expect("cargo fixture should parse");
    let mut results = Vec::new();
    check("Cargo.toml", &cargo, &mut results);
    results
}
