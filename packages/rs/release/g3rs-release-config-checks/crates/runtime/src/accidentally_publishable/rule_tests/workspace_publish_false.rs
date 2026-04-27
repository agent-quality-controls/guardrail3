use g3rs_release_config_checks_assertions::accidentally_publishable::rule as assertions;

use super::helpers::run_check_with_workspace;

#[test]
fn skips_when_publish_inherited_false_from_workspace() {
    let cargo = "\
[package]
name = \"member-crate\"
version = \"0.1.0\"
edition = \"2024\"
publish = { workspace = true }
";
    let workspace = "\
[workspace]
members = [\"member\"]

[workspace.package]
publish = false
";
    let results = run_check_with_workspace(cargo, workspace);

    assertions::assert_no_findings(&results);
}
