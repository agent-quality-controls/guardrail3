/// Rule implementation for `aliases`.
mod aliases;
/// Rule implementation for `analysis`.
mod analysis;
/// Rule implementation for `fields`.
mod fields;

pub(crate) use analysis::BoundaryKind;
pub(crate) use analysis::{ParsedGardeFile, analyze, parse_rust_file};
