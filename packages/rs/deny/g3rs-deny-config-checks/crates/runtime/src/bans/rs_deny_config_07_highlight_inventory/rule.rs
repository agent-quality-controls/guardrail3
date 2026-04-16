use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_bans_settings;
use crate::support::findings::inventory;

const ID: &str = "RS-DENY-CONFIG-07";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };
    let (_, _, expected_highlight) = expected_bans_settings();
    let actual = bans.highlight.as_deref();

    if actual.map(str::to_owned) != expected_highlight {
        results.push(inventory(
            ID,
            "highlight differs from baseline",
            format!(
                "`{deny_rel_path}` sets `[bans].highlight = {}`.",
                actual.unwrap_or("<missing>")
            ),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
