use deny_toml_parser::{BanSkipEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{skip_entry_name, skip_entry_reason, warn, error};

const ID: &str = "RS-DENY-23";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };
    if bans.skip.is_empty() {
        return;
    }

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

    for entry in &bans.skip {
        match entry {
            BanSkipEntry::Simple(name) => {
                missing_reason_count += 1;
                results.push(error(
                    ID,
                    "skip entry must use table form",
                    format!(
                        "`{deny_rel_path}` has `[bans.skip]` string entry `{name}`; use table form with a `reason`."
                    ),
                    deny_rel_path,
                ));
                continue;
            }
            BanSkipEntry::Detailed(_) => {}
        }

        let Some(name) = skip_entry_name(entry) else {
            missing_reason_count += 1;
            results.push(error(
                ID,
                "malformed skip entry",
                format!("`{deny_rel_path}` has `[bans.skip]` entry without a valid crate identifier."),
                deny_rel_path,
            ));
            continue;
        };

        let Some(reason) = skip_entry_reason(entry) else {
            missing_reason_count += 1;
            results.push(error(
                ID,
                "skip entry missing reason",
                format!("`{deny_rel_path}` skips `{name}` without a `reason`."),
                deny_rel_path,
            ));
            continue;
        };

        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
                results.push(warn(
                    ID,
                    "skip entry",
                    format!("`{deny_rel_path}` has documented skip entry `{name}`."),
                    deny_rel_path,
                ));
            }
            Err(issue) => {
                weak_reason_count += 1;
                results.push(error(
                    ID,
                    "skip entry reason too weak",
                    format!(
                        "`{deny_rel_path}` skips `{name}` with a weak `reason`: {}.",
                        issue.message()
                    ),
                    deny_rel_path,
                ));
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total > 0 {
        results.push(warn(
            ID,
            "skip entry count",
            format!(
                "`{deny_rel_path}` has {total} skip entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons)."
            ),
            deny_rel_path,
        ));
    }
}
