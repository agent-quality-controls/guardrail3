use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn inventories_nonbaseline_highlight() {
    let deny = config_facts(
        &canonical_deny_toml_service().replace("highlight = \"all\"", "highlight = \"simplest\""),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-11");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "highlight differs from baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `[bans].highlight = simplest`."
    );
    assert!(result.inventory);
}
