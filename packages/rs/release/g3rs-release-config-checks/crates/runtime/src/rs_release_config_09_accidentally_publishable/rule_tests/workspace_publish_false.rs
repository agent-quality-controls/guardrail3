use g3rs_release_config_checks_assertions::rs_release_config_09_accidentally_publishable as assertions;

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
    let input = crate::test_support::config_input_for_crate(cargo, Some(workspace));

    let results = crate::check(&input);

    assertions::assert_no_findings(&results);
}
