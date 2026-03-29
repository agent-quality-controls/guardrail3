#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "local_root.rs"] // reason: test matrix sidecar split by scenario
mod local_root;
#[path = "missing.rs"] // reason: test matrix sidecar split by scenario
mod missing;
#[path = "parse_error.rs"] // reason: test matrix sidecar split by scenario
mod parse_error;
#[path = "wrong.rs"] // reason: test matrix sidecar split by scenario
mod wrong;
