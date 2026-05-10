use g3rs_code_config_checks_assertions::unsafe_code_lint::rule::assert_single_deny_error;

use super::helpers::{cargo_file, run_check};

#[test]
fn emits_error_for_deny() {
    let results = run_check(&[cargo_file(
        "Cargo.toml",
        "[workspace]\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n",
    )]);

    assert_single_deny_error(&results, "Cargo.toml");
}
