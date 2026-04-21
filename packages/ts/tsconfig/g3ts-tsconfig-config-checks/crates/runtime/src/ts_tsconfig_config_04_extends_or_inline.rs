use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{info, missing_inline_flags};

const ID: &str = "TS-TSCONFIG-CONFIG-04";

pub(crate) fn check(input: &G3TsTsconfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let G3TsTsconfigState::Parsed {
        rel_path,
        uses_extends,
        ..
    } = &input.config
    else {
        return;
    };

    if *uses_extends {
        results.push(info(
            ID,
            "tsconfig inherits strict baseline",
            format!("Root `{rel_path}` declares `extends`, so strict baseline can be inherited."),
            rel_path,
        ));
        return;
    }

    let missing = missing_inline_flags(input);
    if missing.is_empty() {
        results.push(info(
            ID,
            "standalone tsconfig carries strict baseline inline",
            format!(
                "Root `{rel_path}` does not use `extends`, but all strict baseline flags are present inline."
            ),
            rel_path,
        ));
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "standalone tsconfig misses inline strict baseline".to_owned(),
        format!(
            "Root `{rel_path}` does not use `extends`, so it must carry the strict baseline inline. Missing or invalid flags: {}.",
            missing.join(", ")
        ),
        Some(rel_path.clone()),
        None,
    ));
}
