use g3rs_code_config_checks_assertions::common::require_single_result;
use g3rs_code_config_checks_assertions::rs_code_config_12_unsafe_code_lint::assert_deny_error;

use super::helpers::{cargo_file, run_check};

#[test]
fn emits_error_for_deny() {
    let results = run_check(vec![cargo_file(
        "Cargo.toml",
        "[workspace]\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n",
    )]);

    let result = require_single_result(&results);
    assert_deny_error(result, "Cargo.toml");
}
