use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn inventories_license_exceptions() {
    let deny = config_facts(&format!(
        "{}\n[[licenses.exceptions]]\nname = \"demo\"\nallow = [\"MIT\"]\n",
        canonical_deny_toml_service()
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-17");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "license exception entry");
    assert_eq!(
        result.message,
        "`deny.toml` has license exception for `demo`."
    );
    assert!(result.inventory);
}
