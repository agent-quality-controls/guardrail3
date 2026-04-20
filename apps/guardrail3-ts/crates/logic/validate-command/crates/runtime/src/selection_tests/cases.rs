use guardrail3_ts_app_types::{SupportedFamily, ValidateRequest};
use guardrail3_ts_validate_command_assertions::selection as assertions;

#[test]
fn selected_families_follow_canonical_order() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![SupportedFamily::Eslint],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[SupportedFamily::Eslint],
    );
}
