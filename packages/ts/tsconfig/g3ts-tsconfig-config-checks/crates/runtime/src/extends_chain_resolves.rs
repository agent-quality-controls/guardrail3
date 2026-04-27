use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{extends_chain_issues, info};

const ID: &str = "g3ts-tsconfig/extends-chain-resolves";

pub(crate) fn check(input: &G3TsTsconfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let G3TsTsconfigState::Parsed { rel_path, .. } = &input.config else {
        return;
    };

    let issues = extends_chain_issues(input);
    if issues.is_empty() {
        results.push(info(
            ID,
            "tsconfig extends chain resolved",
            "All local `extends` entries resolved to parseable tsconfig files.".to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "tsconfig extends chain broken".to_owned(),
        format!(
            "Local `extends` entries could not be resolved cleanly: {}.",
            issues.join("; ")
        ),
        Some(rel_path.clone()),
        None,
    ));
}
