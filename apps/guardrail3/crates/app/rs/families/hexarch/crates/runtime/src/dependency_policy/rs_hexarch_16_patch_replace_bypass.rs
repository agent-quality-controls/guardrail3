use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use std::collections::BTreeMap;

use crate::dependency_facts::PatchEntryFacts;
use crate::inputs::PatchHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-16";

pub fn check(input: &PatchHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let patch = input.patch;
    if patch.target_layer.is_none() {
        push_success(
            results,
            ID,
            format!(
                "patch/replace entry `{}` stays outside the layered tree",
                patch.key
            ),
            format!(
                "`{}` resolves to `{}` outside the owned layered Rust tree, so it does not bypass hexarch layer enforcement.",
                patch.key, patch.resolved_rel_dir
            ),
            Some(patch.cargo_rel_path.clone()),
        );
        return;
    }

    match patch.escape_hatch_reason.as_deref() {
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("patch/replace entry `{}` missing reason", patch.key),
            format!(
                "`{}` resolves to `{}` inside the layered Rust tree and has no matching escape-hatch reason.",
                patch.key, patch.resolved_rel_dir
            ),
            Some(patch.cargo_rel_path.clone()),
            None,
            false,
        )),
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("patch/replace entry `{}` bypasses hexarch dependency checks", patch.key),
                format!(
                    "`{}` resolves to `{}` inside the layered Rust tree. The bypass is documented but still forbidden here.",
                    patch.key, patch.resolved_rel_dir
                ),
                Some(patch.cargo_rel_path.clone()),
                None,
                false,
            )),
            Err(issue) => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("patch/replace entry `{}` reason too weak", patch.key),
                format!(
                    "`{}` resolves to `{}` inside the layered Rust tree with a weak reason: {}.",
                    patch.key,
                    patch.resolved_rel_dir,
                    issue.message()
                ),
                Some(patch.cargo_rel_path.clone()),
                None,
                false,
            )),
        },
    }
}

pub fn check_count<'a>(
    patches: impl IntoIterator<Item = &'a PatchEntryFacts>,
    results: &mut Vec<CheckResult>,
) {
    let mut counts = BTreeMap::<String, usize>::new();
    for patch in patches {
        if patch.target_layer.is_some() {
            *counts.entry(patch.cargo_rel_path.clone()).or_default() += 1;
        }
    }

    for (cargo_rel_path, count) in counts {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "patch/replace entry count".to_owned(),
            format!("`{cargo_rel_path}` has {count} patch/replace escape hatches inside the layered tree."),
            None,
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_16_patch_replace_bypass_tests/mod.rs"]
mod rs_hexarch_16_patch_replace_bypass_tests;
