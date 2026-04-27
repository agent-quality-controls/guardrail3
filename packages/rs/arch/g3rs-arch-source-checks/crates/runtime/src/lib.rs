use proc_macro2 as _;

#[cfg(test)]
use g3rs_arch_source_checks_assertions as _;

mod feature_gated_exports;
mod lib_facade_only;
mod mod_facade_only;
mod no_path_attr;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
