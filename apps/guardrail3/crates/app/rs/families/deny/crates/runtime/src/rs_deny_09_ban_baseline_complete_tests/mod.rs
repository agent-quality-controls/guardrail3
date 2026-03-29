#[path = "fail_closed.rs"] // reason: test matrix sidecar split by scenario
mod fail_closed;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "library_profile.rs"] // reason: test matrix sidecar split by scenario
mod library_profile;
#[path = "managed_wrappers.rs"] // reason: test matrix sidecar split by scenario
mod managed_wrappers;
#[path = "missing_bans.rs"] // reason: test matrix sidecar split by scenario
mod missing_bans;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
