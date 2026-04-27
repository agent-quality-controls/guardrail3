use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::inventory;
use crate::support::identities::feature_entry_name;

const ID: &str = "g3rs-deny/extra-feature-bans-inventory";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };

    for entry in &bans.features {
        let Some(name) = feature_entry_name(entry) else {
            continue;
        };
        if name != "tokio" {
            results.push(inventory(
                ID,
                "extra feature ban",
                format!("`{deny_rel_path}` has extra feature-ban entry for `{name}`."),
                deny_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
