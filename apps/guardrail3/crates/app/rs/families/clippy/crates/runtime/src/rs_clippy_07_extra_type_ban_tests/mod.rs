#[path = "garde_disabled.rs"] // reason: test matrix sidecar split by scenario
mod garde_disabled;
#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "library_profile.rs"] // reason: test matrix sidecar split by scenario
mod library_profile;
#[path = "malformed_policy_context.rs"] // reason: test matrix sidecar split by scenario
mod malformed_policy_context;
#[path = "malformed_section.rs"] // reason: test matrix sidecar split by scenario
mod malformed_section;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "project_specific.rs"] // reason: test matrix sidecar split by scenario
mod project_specific;
