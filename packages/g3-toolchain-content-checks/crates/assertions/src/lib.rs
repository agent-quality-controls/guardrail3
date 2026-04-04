use g3_toolchain_content_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_toolchain_02_channel_and_components;
#[cfg(feature = "checks")]
pub mod rs_toolchain_03_msrv_consistency;
