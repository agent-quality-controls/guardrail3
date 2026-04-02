use crate::{CheckResult, Severity};

use crate::inputs::CfgTestModuleInput;

const ID: &str = "RS-TEST-01";

pub fn check(input: &CfgTestModuleInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.module.has_body {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "inline cfg(test) body absent".to_owned(),
                "Owned `#[cfg(test)]` declarations stay as sidecar modules instead of inline bodies.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.module.line),
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "inline cfg(test) body in src".to_owned(),
        "Production `src/` files must not contain inline `#[cfg(test)] mod ... { ... }` bodies."
            .to_owned(),
        Some(input.file.rel_path.clone()),
        Some(input.module.line),
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod tests;
