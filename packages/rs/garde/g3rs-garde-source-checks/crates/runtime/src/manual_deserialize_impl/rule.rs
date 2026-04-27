use guardrail3_check_types::G3CheckResult;

use crate::support::{ManualDeserializeImplSite, error};

const ID: &str = "g3rs-garde/manual-deserialize-impl";

pub(crate) fn check(target: &ManualDeserializeImplSite, results: &mut Vec<G3CheckResult>) {
    if !target.needs_validate || target.has_validate {
        return;
    }

    results.push(error(
        ID,
        format!(
            "manual Deserialize impl for `{}` without Validate",
            target.type_name
        ),
        format!(
            "Manual `Deserialize` impl for `{}` bypasses derive-based garde checks and the type does not also implement `Validate`. Add `#[derive(Validate)]` or `impl garde::Validate for {}`.",
            target.type_name, target.type_name
        ),
        &target.rel_path,
        Some(target.line),
    ));
}

#[cfg(test)]
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for target in &input.manual_deserialize_impls {
        check(target, &mut results);
    }
    results
}

#[cfg(test)]
struct Fixture(g3rs_garde_types::G3RsGardeSourceChecksInput);

#[cfg(test)]
impl Fixture {
    fn run(&self) -> Vec<G3CheckResult> {
        check_input(&self.0)
    }
}

#[cfg(test)]
fn fixture(manual_deserialize_impls: Vec<crate::support::ManualDeserializeImplSite>) -> Fixture {
    let mut input = crate::support::active_source_input();
    input.manual_deserialize_impls = manual_deserialize_impls;
    Fixture(input)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
