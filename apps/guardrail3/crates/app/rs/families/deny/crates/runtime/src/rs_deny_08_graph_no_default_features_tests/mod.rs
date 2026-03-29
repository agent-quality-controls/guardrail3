#[path = "fail_closed.rs"] // reason: test matrix sidecar split by scenario
mod fail_closed;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "wrong_value.rs"] // reason: test matrix sidecar split by scenario
mod wrong_value;
