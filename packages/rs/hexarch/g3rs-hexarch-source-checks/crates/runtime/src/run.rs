use g3rs_hexarch_source_checks_types::G3RsHexarchSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHexarchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_hexarch_22_ports_trait_dominance::check(&input.crate_facts, &mut results);
    crate::rs_hexarch_23_adapter_pub_trait::check(&input.crate_facts, &mut results);

    results
}
