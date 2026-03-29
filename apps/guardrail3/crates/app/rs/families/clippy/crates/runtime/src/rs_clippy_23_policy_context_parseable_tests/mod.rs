#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing_content.rs"] // reason: test matrix sidecar split by scenario
mod missing_content;
#[path = "parse_error.rs"] // reason: test matrix sidecar split by scenario
mod parse_error;
#[path = "shape_error.rs"] // reason: test matrix sidecar split by scenario
mod shape_error;
