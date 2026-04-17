use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::CfgTestModuleInput;

const ID: &str = "RS-TEST-SOURCE-01";

pub(crate) fn check(input: &CfgTestModuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if !input.module.has_body {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "inline cfg(test) body absent".to_owned(),
                "Owned `#[cfg(test)]` declarations stay as sidecar modules instead of inline bodies.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.module.line),
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "inline cfg(test) body in src".to_owned(),
        "Production `src/` files must not contain inline `#[cfg(test)] mod ... { ... }` bodies. Move the test module to a sidecar `tests/mod.rs` inside the module directory."
            .to_owned(),
        Some(input.file.rel_path.clone()),
        Some(input.module.line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
