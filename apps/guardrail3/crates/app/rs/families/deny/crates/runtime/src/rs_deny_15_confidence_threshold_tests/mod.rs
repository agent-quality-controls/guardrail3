#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "invalid_or_missing.rs"] // reason: test matrix sidecar split by scenario
mod invalid_or_missing;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "stricter.rs"] // reason: test matrix sidecar split by scenario
mod stricter;
#[path = "weaker.rs"] // reason: test matrix sidecar split by scenario
mod weaker;
