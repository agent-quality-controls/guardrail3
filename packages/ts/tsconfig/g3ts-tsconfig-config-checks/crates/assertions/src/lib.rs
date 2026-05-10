//! Test assertion helpers for the g3ts tsconfig check family.

use g3ts_tsconfig_config_checks_runtime as _;

/// Finding-shape and assert helpers used by integration tests.
#[cfg(feature = "checks")]
pub mod run;
