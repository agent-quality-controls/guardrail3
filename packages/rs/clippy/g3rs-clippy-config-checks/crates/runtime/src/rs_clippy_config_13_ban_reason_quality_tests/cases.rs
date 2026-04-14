use crate::rs_clippy_config_13_ban_reason_quality::check;
use crate::test_support::{findings, input_from_raw};

#[test]
fn errors_on_plain_string_ban_entries_without_reason() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [\"serde_json::from_str\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "ban entry missing reason"
            && finding.message.contains("serde_json::from_str")
    }));
}

#[test]
fn inventories_reasoned_table_ban_entries() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [{ path = \"serde_json::from_str\", reason = \"Use typed boundary parsing\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-13".to_owned(),
            severity: guardrail3_check_types::G3Severity::Info,
            title: "ban entries use reasoned table format".to_owned(),
            message: "All managed ban entries use table format with a `reason` field.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: true,
        }]
    );
}

#[test]
fn errors_on_malformed_macro_ban_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "ban section malformed" && finding.message.contains("disallowed-macros[0]")
    }));
}
