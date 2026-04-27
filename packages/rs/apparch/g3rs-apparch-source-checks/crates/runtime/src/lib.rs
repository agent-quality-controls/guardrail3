#[cfg(test)]
use g3rs_apparch_source_checks_assertions as _;

mod io_traits_in_types;
mod run;
mod types_public_surface;

#[cfg(feature = "checks")]
pub use run::check;
