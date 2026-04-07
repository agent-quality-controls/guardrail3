use g3rs_toolchain_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_toolchain_config_01_channel_and_components;
#[cfg(feature = "checks")]
pub mod rs_toolchain_config_02_msrv_consistency;
