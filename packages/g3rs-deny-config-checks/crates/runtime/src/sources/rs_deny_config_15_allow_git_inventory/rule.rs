use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{inventory, warn};

const ID: &str = "RS-DENY-CONFIG-15";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(sources) = deny.sources.as_ref() else {
        return;
    };

    if !sources.allow_git.is_empty() {
        results.push(warn(
            ID,
            "allow-git is non-empty",
            format!("`{deny_rel_path}` has non-empty `[sources].allow-git`."),
            deny_rel_path,
        ));
    }

    for entry in &sources.allow_git {
        if entry.trim().is_empty() {
            results.push(warn(
                ID,
                "allow-git entry must be non-empty",
                format!("`{deny_rel_path}` has blank `[sources].allow-git` entry."),
                deny_rel_path,
            ));
            continue;
        }
        results.push(inventory(
            ID,
            "allow-git entry",
            format!("`{deny_rel_path}` allows git source `{entry}`."),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
