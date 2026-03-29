#[path = "fail_closed.rs"] // reason: test matrix sidecar split by scenario
mod fail_closed;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "incomplete_baseline.rs"] // reason: test matrix sidecar split by scenario
mod incomplete_baseline;
#[path = "owned_by_specific_rules.rs"] // reason: test matrix sidecar split by scenario
mod owned_by_specific_rules;
#[path = "root_config_is_not_local_policy.rs"] // reason: test matrix sidecar split by scenario
mod root_config_is_not_local_policy;
