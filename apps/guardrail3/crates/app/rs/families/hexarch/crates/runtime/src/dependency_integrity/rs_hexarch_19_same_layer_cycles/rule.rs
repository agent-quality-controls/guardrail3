use guardrail3_domain_report::{CheckResult, Severity};


use crate::dependency_facts::{CycleFacts, MemberDependencyFacts};
use crate::inventory::push_success;

use crate::inputs::CycleHexarchInput;

const ID: &str = "RS-HEXARCH-19";

pub fn check(input: &CycleHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let cycle = input.cycle;
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("same-layer {} dependency cycle", cycle.layer.label()),
        format!(
            "Found same-layer dependency cycle in `{}` layer: {}. Break the cycle by extracting shared code into a separate crate or removing one of the dependencies.",
            cycle.layer.label(),
            cycle.members.join(" -> ")
        ),
        None,
        None,
        false,
    ));
}

pub fn check_inventory(
    members: &[MemberDependencyFacts],
    cycles: &[CycleFacts],
    results: &mut Vec<CheckResult>,
) {
    if members.is_empty() || !cycles.is_empty() {
        return;
    }

    push_success(
        results,
        ID,
        "no same-layer dependency cycles detected".to_owned(),
        format!(
            "Hexarch checked {} workspace member(s) and found no same-layer dependency cycles.",
            members.len()
        ),
        None,
    );
}

