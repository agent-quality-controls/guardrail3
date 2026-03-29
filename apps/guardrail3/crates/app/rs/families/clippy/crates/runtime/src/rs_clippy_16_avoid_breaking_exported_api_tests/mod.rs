#[path = "explicit_false.rs"] // reason: test matrix sidecar split by scenario
mod explicit_false;
#[path = "malformed_policy_context.rs"] // reason: test matrix sidecar split by scenario
mod malformed_policy_context;
#[path = "missing.rs"] // reason: test matrix sidecar split by scenario
mod missing;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "published_library.rs"] // reason: test matrix sidecar split by scenario
mod published_library;
#[path = "published_library_workspace.rs"] // reason: test matrix sidecar split by scenario
mod published_library_workspace;
#[path = "warn_true.rs"] // reason: test matrix sidecar split by scenario
mod warn_true;
