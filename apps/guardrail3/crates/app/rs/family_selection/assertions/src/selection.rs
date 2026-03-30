use guardrail3_app_rs_family_selection::{
    config_for_enabled_family_filtering_for_tests, config_for_explicit_arch_request_for_tests,
    explicit_arch_request_for_tests, minimal_tree_for_tests,
};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub fn minimal_tree() -> guardrail3_domain_project_tree::ProjectTree {
    minimal_tree_for_tests()
}

pub fn config_for_explicit_arch_request() -> guardrail3_domain_config::types::GuardrailConfig {
    config_for_explicit_arch_request_for_tests()
}

pub fn config_for_enabled_family_filtering() -> guardrail3_domain_config::types::GuardrailConfig {
    config_for_enabled_family_filtering_for_tests()
}

pub fn explicit_arch_request() -> Vec<RustValidateFamily> {
    explicit_arch_request_for_tests()
}

pub fn assert_explicit_request_bypasses_disabled_config_filter(selection: &RustFamilySelection) {
    assert!(
        selection.contains(RustValidateFamily::Arch),
        "explicitly requested family should survive disabled config"
    );
}

pub fn assert_enabled_family_filtering(selection: &RustFamilySelection) {
    assert!(
        !selection.contains(RustValidateFamily::Arch),
        "unrequested disabled family should stay filtered"
    );
    assert!(
        selection.contains(RustValidateFamily::Fmt),
        "enabled family should still be selected"
    );
}
