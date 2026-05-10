#[cfg(feature = "checks")]
use g3rs_toolchain_config_checks_runtime as _;
use guardrail3_check_types as _;

/// Shared assertion helpers and macros.
mod common;

#[cfg(feature = "checks")]
pub mod channel_and_components;
#[cfg(feature = "checks")]
pub mod msrv_consistency;
