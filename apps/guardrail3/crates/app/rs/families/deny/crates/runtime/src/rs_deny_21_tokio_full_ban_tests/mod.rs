#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing_full.rs"] // reason: test matrix sidecar split by scenario
mod missing_full;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "wrong_allow_list.rs"] // reason: test matrix sidecar split by scenario
mod wrong_allow_list;
