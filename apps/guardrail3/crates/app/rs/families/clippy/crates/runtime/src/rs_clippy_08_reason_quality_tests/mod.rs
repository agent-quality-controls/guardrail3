#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "malformed_sections.rs"] // reason: test matrix sidecar split by scenario
mod malformed_sections;
#[path = "missing_reasons.rs"] // reason: test matrix sidecar split by scenario
mod missing_reasons;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "user_added_reasoned_bans.rs"] // reason: test matrix sidecar split by scenario
mod user_added_reasoned_bans;
