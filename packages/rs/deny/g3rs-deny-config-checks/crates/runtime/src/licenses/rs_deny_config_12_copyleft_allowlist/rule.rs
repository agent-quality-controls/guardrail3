use std::collections::BTreeSet;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;

const ID: &str = "RS-DENY-CONFIG-12";
const COPYLEFT_LICENSES: &[&str] = &[
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "AGPL-3.0",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "LGPL-2.1",
    "LGPL-3.0",
    "SSPL-1.0",
    "EUPL-1.2",
];

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(licenses) = deny.licenses.as_ref() else {
        return;
    };

    for license in licenses.allow.iter().cloned().collect::<BTreeSet<_>>() {
        if COPYLEFT_LICENSES.contains(&license.as_str()) {
            results.push(warn(
                ID,
                "copyleft license allowed",
                format!("`{deny_rel_path}` allows copyleft license `{license}`."),
                deny_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
