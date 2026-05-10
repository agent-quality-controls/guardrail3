/// Constants shared across surface modules.
mod constants;
/// `ESLint` config surface state and reader.
mod eslint;
/// Common helpers re-exported by surface modules.
mod prelude;
/// App-root and rel-path computations.
mod roots;

pub use eslint::{G3TsAstroRawEslintConfigState, read_eslint_config_surface};
pub use roots::{app_relative_path, is_under_app_root, nearest_app_root, scoped_rel_path};
