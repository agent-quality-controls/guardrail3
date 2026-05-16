use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/deprecated-advisories";

/// Runs the rule and appends any findings to `results`.
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
                format!("`{deny_rel_path}` uses deprecated `[advisories].{deprecated}`."),
                deny_rel_path,
            ));
        }
    }
}
