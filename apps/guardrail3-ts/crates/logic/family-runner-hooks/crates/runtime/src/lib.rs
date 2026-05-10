/// Runs the TS hook family.
mod run;
/// Toolchain-gate construction and execution after the static rule pipeline.
mod toolchain_gates;

#[cfg(feature = "api")]
pub use run::run;
#[cfg(feature = "api")]
pub use toolchain_gates::{
    ToolchainGate, ToolchainOutcome, run_toolchain_gates, toolchain_gate_list,
};
