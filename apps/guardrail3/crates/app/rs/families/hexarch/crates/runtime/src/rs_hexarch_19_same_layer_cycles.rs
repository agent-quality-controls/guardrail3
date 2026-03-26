use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::CycleHexarchInput;

const ID: &str = "RS-HEXARCH-19";

pub fn check(input: &CycleHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let cycle = input.cycle;
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("same-layer {} dependency cycle", cycle.layer.label()),
        message: format!(
            "Found same-layer dependency cycle in `{}` layer: {}",
            cycle.layer.label(),
            cycle.members.join(" -> ")
        ),
        file: None,
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_19_same_layer_cycles_tests/mod.rs"]
mod rs_hexarch_19_same_layer_cycles_tests;
