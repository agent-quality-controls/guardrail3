use super::super::{check};
use guardrail3_domain_report::CheckResult;
use crate::facts::RustfmtConfigKind;
use crate::inputs::RustfmtExtraConfigInput;
pub(super) fn run_check(config_rel: &str, config_kind: RustfmtConfigKind) -> Vec<CheckResult> {
    let input = RustfmtExtraConfigInput {
        config_rel: config_rel.to_owned(),
        config_kind,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
pub(super) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
