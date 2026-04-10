#[cfg(test)]
use g3rs_hexarch_source_checks_assertions as _;

mod rs_hexarch_22_ports_trait_dominance;
mod rs_hexarch_23_adapter_pub_trait;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
