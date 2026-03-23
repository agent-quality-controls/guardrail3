use crate::domain::report::{CheckResult, Severity};

use super::inputs::PatchHexarchInput;

const ID: &str = "RS-HEXARCH-16";

pub fn check(input: &PatchHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let patch = input.patch;
    if patch.target_layer.is_none() {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("patch/replace entry `{}` bypasses hexarch dependency checks", patch.key),
        message: format!(
            "`{}` resolves to `{}` inside the layered Rust tree. `patch`/`replace` path overrides bypass normal dependency-direction checks and are forbidden here.",
            patch.key, patch.resolved_rel_dir
        ),
        file: Some(patch.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_16_patch_replace_bypass_tests/mod.rs"]
mod tests;
