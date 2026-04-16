use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{parse_ban_section, raw_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-13";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    let mut issue_count = 0usize;
    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let section = parse_ban_section(parsed, key);
        for malformed in &section.malformed_messages {
            issue_count += 1;
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "ban section malformed".to_owned(),
                malformed.clone(),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
        for entry in section.entries {
            if entry.is_plain_string || entry.reason.as_deref().is_none() {
                issue_count += 1;
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "ban entry missing reason".to_owned(),
                    format!(
                        "`{}` in `{key}` must use table format with a `reason` field.",
                        entry.path
                    ),
                    Some(input.clippy_rel_path.clone()),
                    None,
                ));
            }
        }
    }

    if issue_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "ban entries use reasoned table format".to_owned(),
                "All managed ban entries use table format with a `reason` field.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_13_ban_reason_quality_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_clippy_config_13_ban_reason_quality_tests;
