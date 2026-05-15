use guardrail3_rs_app_types::{SupportedFamily, ValidateWorkspaceRequest};
use guardrail3_rs_validate_command_assertions::selection as assertions;

#[test]
fn selected_families_follow_canonical_order() {
    let request = ValidateWorkspaceRequest {
        workspace_root: "ignored".into(),
        families: vec![
            SupportedFamily::Release,
            SupportedFamily::Fmt,
            SupportedFamily::Toolchain,
        ],
        include_inventory: false,
        staged: false,
        rules_only: true,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        vec![
            SupportedFamily::Toolchain,
            SupportedFamily::Fmt,
            SupportedFamily::Release,
        ]
        .as_slice(),
    );
}
