use g3rs_release_config_checks_assertions::rs_release_config_01_description_present::rule as assertions;

use super::helpers::run_check_with_workspace;

#[test]
fn accepts_workspace_inherited_description() {
    let member = "\
[package]
name = \"member-crate\"
version = \"0.1.0\"
edition = \"2024\"
description = { workspace = true }
";
    let workspace = "\
[workspace]
members = [\"member\"]

[workspace.package]
description = \"shared workspace description\"
";

    let results = run_check_with_workspace(member, Some(workspace));

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "member-crate: description present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
