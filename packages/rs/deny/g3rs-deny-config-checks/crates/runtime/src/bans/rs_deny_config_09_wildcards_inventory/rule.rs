use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_bans_settings;
use crate::support::findings::warn;

const ID: &str = "RS-DENY-CONFIG-09";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };

    let (expected, _, _) = expected_bans_settings();
    let actual = bans.wildcards.as_deref();
    if actual.map(str::to_owned) != expected {
        results.push(warn(
            ID,
            "wildcards differs from baseline",
            format!(
                "`{deny_rel_path}` sets `[bans].wildcards = {}`.",
                actual.unwrap_or("<missing>")
            ),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
