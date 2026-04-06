use deny_toml_parser::parse as parse_deny_toml;
use guardrail3_check_types::G3CheckResult;

use crate::bans::rs_deny_config_16_tokio_full_ban::check;

pub(super) fn run_check(deny_toml: &str) -> Vec<G3CheckResult> {
    let deny = parse_deny_toml(deny_toml).expect("deny fixture should parse");
    let mut results = Vec::new();
    check("deny.toml", &deny, &mut results);
    results
}
