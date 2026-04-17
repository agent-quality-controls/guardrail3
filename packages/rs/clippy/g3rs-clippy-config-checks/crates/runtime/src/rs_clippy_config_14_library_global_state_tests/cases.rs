use crate::rs_clippy_config_14_library_global_state::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_14_library_global_state as assertions;
use guardrail3_rs_toml_parser::types::RustProfile;
use test_support::{input_with_raw, missing_cargo_root, parsed_rust_policy};

fn library_global_state_baseline() -> &'static str {
    r#"
disallowed-types = [
  { path = "std::sync::LazyLock", reason = "ban global state" },
  { path = "std::sync::OnceLock", reason = "ban global state" },
  { path = "once_cell::sync::Lazy", reason = "ban global state" },
  { path = "once_cell::sync::OnceCell", reason = "ban global state" },
]
"#
}

#[test]
fn reports_missing_library_global_state_bans() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-types = []\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_missing_global_state_ban_count(&results, 4);
    assertions::assert_contains_missing_global_state_ban(&results, "std::sync::LazyLock");
}

#[test]
fn inventories_complete_library_global_state_bans() {
    let input = input_with_raw(
        "clippy.toml",
        library_global_state_baseline(),
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::info(
            "library global-state bans present",
            "Library profile includes all managed global-state type bans.",
            "clippy.toml",
            true,
        )]
    );
}
