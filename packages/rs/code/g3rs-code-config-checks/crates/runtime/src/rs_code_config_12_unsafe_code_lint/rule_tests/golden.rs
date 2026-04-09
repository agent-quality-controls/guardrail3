use g3rs_code_config_checks_assertions::common::require_single_result;
use g3rs_code_config_checks_assertions::rs_code_config_12_unsafe_code_lint::assert_forbid_inventory_info;
use g3rs_code_config_checks_types::G3RsCodeUnsafeCodeLintFact;

use super::helpers::run_check;

#[test]
fn emits_inventory_info_for_forbid() {
    let results = run_check(vec![G3RsCodeUnsafeCodeLintFact {
        cargo_rel_path: "Cargo.toml".to_owned(),
        lint_level: Some("forbid".to_owned()),
    }]);

    let result = require_single_result(&results);
    assert_forbid_inventory_info(result, "Cargo.toml");
}
