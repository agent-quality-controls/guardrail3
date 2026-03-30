#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "malformed_policy_context.rs"] // reason: test matrix sidecar split by scenario
mod malformed_policy_context;
#[path = "missing_global_state.rs"] // reason: test matrix sidecar split by scenario
mod missing_global_state;
#[path = "package_workspace_profile.rs"] // reason: test matrix sidecar split by scenario
mod package_workspace_profile;
#[path = "parity.rs"] // reason: test matrix sidecar split by scenario
mod parity;
#[path = "standalone_app_profile.rs"] // reason: test matrix sidecar split by scenario
mod standalone_app_profile;
