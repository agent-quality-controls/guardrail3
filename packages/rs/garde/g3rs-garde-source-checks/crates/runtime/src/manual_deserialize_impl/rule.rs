use guardrail3_check_types::G3CheckResult;

use crate::support::{ManualDeserializeImplSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/manual-deserialize-impl";

/// Runs the rule and appends any findings to `results`.
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
