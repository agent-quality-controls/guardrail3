use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{
    effective_check_actionable, effective_flag_mismatches, has_external_extends, info,
};

const ID: &str = "TS-TSCONFIG-CONFIG-05";

pub(crate) fn check(input: &G3TsTsconfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let G3TsTsconfigState::Parsed { rel_path, .. } = &input.config else {
        return;
    };

    if !effective_check_actionable(input) {
        if has_external_extends(input) {
            results.push(info(
                ID,
                "strict tsconfig baseline deferred through external extends",
                "The current wave accepts external `extends` parents without proving their effective strict baseline."
                    .to_owned(),
                rel_path,
            ));
        }
        return;
    }

    let mismatches = effective_flag_mismatches(input);
    if mismatches.is_empty() {
        results.push(info(
            ID,
            "strict tsconfig baseline enforced",
            "The effective tsconfig keeps the required 12 strict boolean flags.".to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "strict tsconfig baseline weakened".to_owned(),
        format!(
            "The effective tsconfig does not keep the required strict baseline. Problems: {}.",
            mismatches.join(", ")
        ),
        Some(rel_path.clone()),
        None,
    ));
}
