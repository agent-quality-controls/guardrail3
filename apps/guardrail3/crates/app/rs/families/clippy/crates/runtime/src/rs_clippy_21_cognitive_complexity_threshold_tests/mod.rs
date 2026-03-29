#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing.rs"] // reason: test matrix sidecar split by scenario
mod missing;
#[path = "parse_error.rs"] // reason: test matrix sidecar split by scenario
mod parse_error;
#[path = "wrong.rs"] // reason: test matrix sidecar split by scenario
mod wrong;
