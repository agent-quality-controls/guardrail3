use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_forbid_inventory_info(result: &G3CheckResult, rel_path: &str) {
    assert_eq!(result.id(), "RS-CODE-12");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "unsafe_code = forbid");
    assert_eq!(
        result.message(),
        "unsafe_code is set to forbid in workspace lints."
    );
    assert_eq!(result.file(), Some(rel_path));
    assert_eq!(result.line(), None);
    assert!(result.inventory(), "forbid inventory should stay hidden by default");
}

pub fn assert_deny_error(result: &G3CheckResult, rel_path: &str) {
    assert_eq!(result.id(), "RS-CODE-12");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "unsafe_code should be forbid");
    assert_eq!(
        result.message(),
        "unsafe_code = deny can be overridden; use forbid in workspace lints."
    );
    assert_eq!(result.file(), Some(rel_path));
    assert_eq!(result.line(), None);
    assert!(!result.inventory(), "deny should stay in normal output");
}
