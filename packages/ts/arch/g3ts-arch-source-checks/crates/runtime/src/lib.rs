/// Rule: only facade files may re-export from a package.
mod facade_only;
/// Rule: facade files must be parseable as re-export only.
mod facade_parseable;
/// Rule: facade files may not re-export the whole package surface.
mod no_broad_reexport;
/// Aggregates per-rule checks for the arch source checks family.
mod run;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_source_checks_assertions as _;
