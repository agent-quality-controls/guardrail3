#[path = "fail_closed.rs"] // reason: test matrix sidecar split by scenario
mod fail_closed;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing_value.rs"] // reason: test matrix sidecar split by scenario
mod missing_value;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "weaker_value.rs"] // reason: test matrix sidecar split by scenario
mod weaker_value;
