use guardrail3_app_rs_family_toolchain as _;

#[cfg(feature = "checks")]
pub mod rs_toolchain_01_exists;
#[cfg(feature = "checks")]
pub mod rs_toolchain_02_channel_and_components;
#[cfg(feature = "checks")]
pub mod rs_toolchain_03_msrv_consistency;
#[cfg(feature = "checks")]
pub mod rs_toolchain_04_legacy_file;
