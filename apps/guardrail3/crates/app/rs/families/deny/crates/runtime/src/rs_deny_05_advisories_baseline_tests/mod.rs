#[path = "fail_closed.rs"] // reason: test matrix sidecar split by scenario
mod fail_closed;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing_values.rs"] // reason: test matrix sidecar split by scenario
mod missing_values;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "wrong_values.rs"] // reason: test matrix sidecar split by scenario
mod wrong_values;
