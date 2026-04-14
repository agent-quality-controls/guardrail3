use std::collections::BTreeMap;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{parse_ban_section, raw_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-16";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    let mut issue_count = 0usize;
    for key in ["disallowed-methods", "disallowed-types", "disallowed-macros"] {
        let section = parse_ban_section(parsed, key);
        for malformed in &section.malformed_messages {
            issue_count += 1;
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "ban section malformed".to_owned(),
                malformed.clone(),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
        let mut counts = BTreeMap::new();
        for entry in section.entries {
            *counts.entry(entry.path).or_insert(0usize) += 1;
        }
        for (path, count) in counts {
            if count > 1 {
                issue_count += 1;
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "duplicate ban entry".to_owned(),
                    format!(
                        "`{path}` appears {count} times in `{key}`. Remove the duplicate entries."
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
                "ban entries are duplicate-free".to_owned(),
                "Managed ban sections contain no duplicate paths.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
