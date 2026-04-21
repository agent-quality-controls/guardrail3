mod run;
mod ts_arch_source_01_facade_parseable;
mod ts_arch_source_02_facade_only;
mod ts_arch_source_03_no_broad_reexport;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_source_checks_assertions as _;
