use guardrail3_check_types::G3CheckResult;

pub fn has_rule(results: &[G3CheckResult], rule_id: &str) -> bool {
    results.iter().any(|result| result.id() == rule_id)
}

pub fn finding_ids(results: &[G3CheckResult]) -> Vec<&str> {
    results.iter().map(|result| result.id()).collect()
}
