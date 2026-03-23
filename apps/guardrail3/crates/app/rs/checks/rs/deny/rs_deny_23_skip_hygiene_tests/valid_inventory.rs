use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn inventories_supported_skip_entry_shapes() {
    let config = config_facts(&skip_toml(
        r#"["plain-crate", { crate = "serde@1.0.0", reason = "good enough reason text" }, { name = "windows-sys", version = "0.60.2", reason = "good enough reason text" }]"#,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "`deny.toml` has skip entry `plain-crate`.".to_owned(),
        "`deny.toml` has skip entry `serde`.".to_owned(),
        "`deny.toml` has skip entry `windows-sys`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-23"
            && result.severity == Severity::Info
            && result.inventory
            && result.title == "skip entry"
            && result.file.as_deref() == Some("deny.toml")
    }));
}
