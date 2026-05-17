use g3ts_apparch_types::G3TsApparchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::types_dependency_direction::check(input, &mut results);
    crate::logic_dependency_direction::check(input, &mut results);
    crate::io_outbound_dependency_direction::check(input, &mut results);
    crate::io_inbound_dependency_direction::check(input, &mut results);
    crate::app_no_direct_outbound::check(input, &mut results);
    crate::types_purity::check(input, &mut results);
    crate::logic_purity::check(input, &mut results);
    results
}
