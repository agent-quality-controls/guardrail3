use crate::missing_type_ban::check;
use g3rs_clippy_config_checks_assertions::missing_type_ban as assertions;
use guardrail3_rs_toml_parser::types::RustProfile;
use test_support::{input_from_raw, input_with_raw, missing_cargo_root, parsed_rust_policy};

#[test]
fn reports_missing_baseline_type_ban() {
    let input = input_from_raw("clippy.toml", "disallowed-types = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_missing_type_ban_count(&results, 21);
    assertions::assert_contains_for_path(&results, "std::collections::HashMap");
}

#[test]
fn reports_library_profile_specific_missing_type_ban() {
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

    assertions::assert_contains_for_path(&results, "std::sync::Mutex");
}

#[test]
fn drops_garde_owned_type_bans_when_garde_is_disabled() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-types = []\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), false),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_missing_type_ban_count(&results, 6);
}
