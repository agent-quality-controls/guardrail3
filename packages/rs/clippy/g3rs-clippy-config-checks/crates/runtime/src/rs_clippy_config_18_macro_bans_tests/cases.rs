use crate::rs_clippy_config_18_macro_bans::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_18_macro_bans as assertions;
use test_support::input_from_raw;

#[test]
fn reports_missing_macro_bans() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_contains_missing_macro_ban(&results, "println!");
}

#[test]
fn reports_malformed_macro_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_contains_malformed_macro_section(&results, "disallowed-macros[0]");
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

    assertions::assert_macro_ban_present_count(&results, 5);
}
