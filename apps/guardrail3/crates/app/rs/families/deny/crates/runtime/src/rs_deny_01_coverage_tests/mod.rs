#[path = "local_cargo_variant.rs"] // reason: test matrix sidecar split by scenario
mod local_cargo_variant;
#[path = "multi_root_coverage.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_coverage;
#[path = "parse_error.rs"] // reason: test matrix sidecar split by scenario
mod parse_error;
#[path = "precedence.rs"] // reason: test matrix sidecar split by scenario
mod precedence;
#[path = "selective_uncovered.rs"] // reason: test matrix sidecar split by scenario
mod selective_uncovered;
