use super::check;
use crate::support::EXPECTED_MACRO_BANS;
use crate::test_support::{findings, input_from_raw};

#[test]
fn reports_missing_macro_bans() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "missing macro ban" && finding.message.contains("println!")
    }));
}

#[test]
fn reports_malformed_macro_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "disallowed-macros section malformed"
            && finding.message.contains("disallowed-macros[0]")
    }));
}

#[test]
fn inventories_full_macro_baseline() {
    let input = input_from_raw(
        "clippy.toml",
        r#"
disallowed-macros = [
  { path = "std::println", reason = "no println" },
  { path = "std::eprintln", reason = "no eprintln" },
  { path = "std::dbg", reason = "no dbg" },
  { path = "std::todo", reason = "no todo" },
  { path = "std::unimplemented", reason = "no unimplemented" },
]
"#,
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let present = findings
        .iter()
        .filter(|finding| finding.title == "macro ban present")
        .count();
    assert_eq!(present, EXPECTED_MACRO_BANS.len(), "{findings:#?}");
}
