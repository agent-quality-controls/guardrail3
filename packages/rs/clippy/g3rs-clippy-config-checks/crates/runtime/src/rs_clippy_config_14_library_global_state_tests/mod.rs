use crate::rs_clippy_config_14_library_global_state::check;
use crate::test_support::{baseline_toml, findings, input_with_raw, parsed_rust_policy};
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn reports_missing_library_global_state_bans() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-types = []\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let missing = findings
        .iter()
        .filter(|finding| finding.title == "library clippy.toml missing global-state type ban")
        .collect::<Vec<_>>();

    assert_eq!(missing.len(), 4, "{findings:#?}");
    assert!(
        missing
            .iter()
            .any(|finding| finding.message.contains("std::sync::LazyLock"))
    );
}

#[test]
fn inventories_complete_library_global_state_bans() {
    let input = input_with_raw(
        "clippy.toml",
        &baseline_toml(RustProfile::Library, true),
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-14".to_owned(),
            severity: guardrail3_check_types::G3Severity::Info,
            title: "library global-state bans present".to_owned(),
            message: "Library profile includes all managed global-state type bans.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: true,
        }]
    );
}
