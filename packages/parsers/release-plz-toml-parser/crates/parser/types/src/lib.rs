/// Typed release-plz.toml model definitions.
mod release_plz_toml;
use toml as _;

pub use release_plz_toml::{ReleasePlzPackage, ReleasePlzToml, ReleasePlzWorkspace};
