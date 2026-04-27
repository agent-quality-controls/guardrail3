mod facade_only;
mod facade_parseable;
mod no_broad_reexport;
mod run;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_source_checks_assertions as _;
