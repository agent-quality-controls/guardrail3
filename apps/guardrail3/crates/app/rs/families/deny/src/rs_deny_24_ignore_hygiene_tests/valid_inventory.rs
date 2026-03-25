use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn inventories_supported_ignore_entry_shapes() {
    let config = config_facts(&ignore_toml(
        r#"["RUSTSEC-2026-0000", { id = "RUSTSEC-2026-0001", reason = "good enough reason text" }]"#,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "`deny.toml` ignores advisory `RUSTSEC-2026-0000`.".to_owned(),
        "`deny.toml` ignores advisory `RUSTSEC-2026-0001`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-24"
            && result.severity == Severity::Info
            && result.inventory
            && result.title == "advisory ignore entry"
            && result.file.as_deref() == Some("deny.toml")
    }));
}
