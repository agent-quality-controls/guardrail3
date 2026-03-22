use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn inventories_extra_feature_bans() {
    let config = config_facts(&format!(
        "{}\n[[bans.features]]\nname = \"serde\"\ndeny = [\"derive\"]\n",
        canonical_deny_toml_service()
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-22"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "extra feature ban"
            && result.message.contains("serde")
            && result.file.as_deref() == Some("deny.toml")
    }));
}
