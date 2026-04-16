use proc_macro2 as _;

#[cfg(test)]
use g3rs_arch_source_checks_assertions as _;

mod rs_arch_02_lib_facade_only;
mod rs_arch_04_mod_facade_only;
mod rs_arch_08a_feature_gated_exports;
mod rs_arch_09_no_path_attr;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
