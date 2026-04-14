use std::collections::BTreeSet;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{EXPECTED_MACRO_BANS, display_macro_name, parse_ban_section, raw_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-18";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-macros");
    for malformed in &section.malformed_messages {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "disallowed-macros section malformed".to_owned(),
            malformed.clone(),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }

    let found: BTreeSet<_> = section.entries.into_iter().map(|entry| entry.path).collect();
    for expected in EXPECTED_MACRO_BANS {
        if found.contains(*expected) {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "macro ban present".to_owned(),
                    format!("`{}!` is banned.", display_macro_name(expected)),
                    Some(input.clippy_rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "missing macro ban".to_owned(),
                format!(
                    "`{}!` is not present in `disallowed-macros`. Add it to `disallowed-macros` in clippy.toml.",
                    display_macro_name(expected)
                ),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
    }
}
