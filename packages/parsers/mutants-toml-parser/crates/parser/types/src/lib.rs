/// Typed `.cargo/mutants.toml` model definitions.
mod mutants_toml;
use toml as _;

pub use mutants_toml::{MutantsToml, Sharding, TestTool};
