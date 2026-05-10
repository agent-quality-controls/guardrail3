use proc_macro2 as _;

#[cfg(test)]
use g3rs_arch_source_checks_assertions as _;

/// feature gated exports module.
mod feature_gated_exports;
/// lib facade only module.
mod lib_facade_only;
/// mod facade only module.
mod mod_facade_only;
/// no path attr module.
mod no_path_attr;
/// run module.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
