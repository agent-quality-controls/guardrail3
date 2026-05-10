#![allow(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "parser tests use direct exact-shape assertions for concise contract proofs"
)]

use cliff_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_fixture, parse_from_tempfile};

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_sections_empty(&cfg);
}

#[test]
fn git_section_with_commit_parsers() {
    let cfg = parse_fixture(
        r#"
[git]
conventional_commits = true
filter_unconventional = true

[[git.commit_parsers]]
message = "^feat"
group = "Features"

[[git.commit_parsers]]
message = "^fix"
group = "Bug Fixes"

[[git.commit_parsers]]
message = "^chore"
skip = true
"#,
    );

    let git = cfg.git.as_ref().expect("git section should be present");
    assertions::assert_bool_field(git.conventional_commits, Some(true), "conventional_commits");
    assertions::assert_bool_field(
        git.filter_unconventional,
        Some(true),
        "filter_unconventional",
    );

    let parsers = git
        .commit_parsers
        .as_ref()
        .expect("commit_parsers should be present");
    assertions::assert_list_len(parsers, 3, "commit_parsers");
    assertions::assert_commit_entry(&parsers[0], Some("^feat"), Some("Features"), None);
    assertions::assert_commit_entry(&parsers[1], Some("^fix"), Some("Bug Fixes"), None);
    assertions::assert_commit_entry(&parsers[2], Some("^chore"), None, Some(true));
}

#[test]
fn commit_parsers_array_deserialized() {
    let cfg = parse_fixture(
        r#"
[[git.commit_parsers]]
message = "^docs"
group = "Documentation"

[[git.commit_parsers]]
message = "^refactor"
group = "Refactoring"
"#,
    );

    let git = cfg.git.as_ref().expect("git section should be present");
    let parsers = git
        .commit_parsers
        .as_ref()
        .expect("commit_parsers should be present");
    assertions::assert_list_len(parsers, 2, "commit_parsers");
    assertions::assert_commit_entry(&parsers[0], Some("^docs"), Some("Documentation"), None);
    assertions::assert_commit_entry(&parsers[1], Some("^refactor"), Some("Refactoring"), None);
}

#[test]
fn changelog_section_parses() {
    let input = concat!(
        "[changelog]\n",
        "header = \"# Changelog\"\n",
        "body = \"{% for commit in commits %}{{ commit.message }}{% endfor %}\"\n",
        "footer = \"<!-- generated -->\"\n",
    );
    let cfg = parse_fixture(input);

    assertions::assert_changelog_fields(
        &cfg,
        Some("# Changelog"),
        Some("<!-- generated -->"),
        true,
    );
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r"
[git]
conventional_commits = true
",
    );

    let git = cfg.git.as_ref().expect("git section should be present");
    assertions::assert_bool_field(git.conventional_commits, Some(true), "conventional_commits");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}

#[test]
fn unknown_top_level_keys_captured_in_extra() {
    let cfg = parse_fixture(
        r#"
[bump]
initial_tag = "0.1.0"
"#,
    );

    assertions::assert_sections_absent(&cfg);
    assertions::assert_top_level_extra_key(&cfg, "bump");
}
