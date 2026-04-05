use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::warn;

const ID: &str = "RS-DENY-04";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };

    for deprecated in ["vulnerability", "notice", "unsound"] {
        let present = match deprecated {
            "vulnerability" => advisories.vulnerability.is_some(),
            "notice" => advisories.notice.is_some(),
            "unsound" => advisories.unsound.is_some(),
            _ => false,
        };
        if present {
            results.push(warn(
                ID,
                format!("deprecated advisory field `{deprecated}`"),
                format!(
                    "`{deny_rel_path}` uses deprecated `[advisories].{deprecated}`."
                ),
                deny_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
