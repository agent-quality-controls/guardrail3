use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_bans_settings;
use crate::support::findings::error;

const ID: &str = "RS-DENY-CONFIG-08";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        results.push(error(
            ID,
            "[bans] section missing",
            format!("`{deny_rel_path}` has no `[bans]` section."),
            deny_rel_path,
        ));
        return;
    };

    let (_, expected, _) = expected_bans_settings();
    match bans.allow_wildcard_paths {
        Some(value) if value == expected => {}
        _ => results.push(error(
            ID,
            "allow-wildcard-paths must be true",
            format!("`{deny_rel_path}` must set `[bans].allow-wildcard-paths = true`."),
            deny_rel_path,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
