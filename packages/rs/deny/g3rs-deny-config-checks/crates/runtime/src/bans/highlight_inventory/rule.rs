use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_bans_settings;
use crate::support::findings::inventory;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/highlight-inventory";

/// Runs the rule and appends any findings to `results`.
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
