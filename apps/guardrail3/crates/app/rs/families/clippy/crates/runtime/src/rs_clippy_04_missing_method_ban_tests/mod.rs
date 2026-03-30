#[path = "garde_disabled.rs"] // reason: test matrix sidecar split by scenario
mod garde_disabled;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "malformed_policy_context.rs"] // reason: test matrix sidecar split by scenario
mod malformed_policy_context;
#[path = "malformed_section.rs"] // reason: test matrix sidecar split by scenario
mod malformed_section;
#[path = "missing_bans.rs"] // reason: test matrix sidecar split by scenario
mod missing_bans;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "plain_string_presence.rs"] // reason: test matrix sidecar split by scenario
mod plain_string_presence;
