use g3ts_arch_types::G3TsArchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsArchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::facade_parseable::check(input, &mut results);
    crate::facade_only::check(input, &mut results);
    crate::no_broad_reexport::check(input, &mut results);
    results
}
