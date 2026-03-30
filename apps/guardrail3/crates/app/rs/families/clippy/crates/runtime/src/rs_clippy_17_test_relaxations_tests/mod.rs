#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "missing.rs"] // reason: test matrix sidecar split by scenario
mod missing;
#[path = "multiple_relaxations.rs"] // reason: test matrix sidecar split by scenario
mod multiple_relaxations;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "wrong_type.rs"] // reason: test matrix sidecar split by scenario
mod wrong_type;
