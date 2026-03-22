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

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-22");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra feature ban");
    assert_eq!(
        result.message,
        "`deny.toml` has extra feature-ban entry for `serde`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
