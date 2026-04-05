use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::warn;

const ID: &str = "RS-DENY-10";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        results.push(warn(
            ID,
            "[bans] section missing",
            format!("`{deny_rel_path}` has no `[bans]` section."),
            deny_rel_path,
        ));
        return;
    };

    match bans.multiple_versions.as_deref() {
        Some("deny") => {}
        Some(other) => results.push(warn(
            ID,
            "multiple-versions weaker than baseline",
            format!("`{deny_rel_path}` sets `[bans].multiple-versions = \"{other}\"`."),
            deny_rel_path,
        )),
        None => results.push(warn(
            ID,
            "multiple-versions missing",
            format!("`{deny_rel_path}` does not set `[bans].multiple-versions`."),
            deny_rel_path,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
