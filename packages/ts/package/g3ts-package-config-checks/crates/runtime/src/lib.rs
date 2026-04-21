mod run;
mod support;
mod ts_package_config_01_root_exists;
mod ts_package_config_02_root_parseable;
mod ts_package_config_03_root_private;
mod ts_package_config_04_root_package_manager;
mod ts_package_config_05_root_engines;
mod ts_package_config_06_root_scripts;
mod ts_package_config_07_root_pnpm;
mod ts_package_config_08_local_banned_dependencies;

#[cfg(feature = "checks")]
pub use run::check;
