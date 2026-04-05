use clippy_toml_parser::parse as parse_clippy_toml;
use g3_clippy_content_checks_types::G3ClippyContentChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::rs_clippy_11_excessive_nesting_threshold::check;

pub(super) fn run_check(clippy_toml: &str) -> Vec<G3CheckResult> {
    let input = G3ClippyContentChecksInput {
        clippy_rel_path: "clippy.toml".to_owned(),
        clippy: parse_clippy_toml(clippy_toml).expect("clippy fixture should parse"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
