use g3rs_code_config_checks_assertions::unsafe_code_lint::rule::assert_single_forbid_inventory_info;

use super::helpers::{cargo_file, run_check};

#[test]
fn emits_inventory_info_for_forbid() {
    let results = run_check(&[cargo_file(
        "Cargo.toml",
        "[workspace]\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
    )]);

    assert_single_forbid_inventory_info(&results, "Cargo.toml");
}
