#[path = "multi_root_coverage.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_coverage;
#[path = "non_rust_roots.rs"] // reason: test matrix sidecar split by scenario
mod non_rust_roots;
#[path = "root_dotfile_without_root_cargo.rs"] // reason: test matrix sidecar split by scenario
mod root_dotfile_without_root_cargo;
#[path = "root_policy_without_root_cargo.rs"] // reason: test matrix sidecar split by scenario
mod root_policy_without_root_cargo;
#[path = "selective_uncovered.rs"] // reason: test matrix sidecar split by scenario
mod selective_uncovered;
