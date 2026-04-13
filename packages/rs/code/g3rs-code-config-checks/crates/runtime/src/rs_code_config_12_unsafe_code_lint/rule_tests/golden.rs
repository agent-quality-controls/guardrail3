use g3rs_code_config_checks_assertions::common::require_single_result;
use g3rs_code_config_checks_assertions::rs_code_config_12_unsafe_code_lint::assert_forbid_inventory_info;

use super::helpers::{cargo_file, run_check};

#[test]
fn emits_inventory_info_for_forbid() {
    let results = run_check(vec![cargo_file(
        "Cargo.toml",
        "[workspace]\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
    )]);

    let result = require_single_result(&results);
    assert_forbid_inventory_info(result, "Cargo.toml");
}
