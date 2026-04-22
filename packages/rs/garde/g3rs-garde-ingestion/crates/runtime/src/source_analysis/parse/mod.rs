mod aliases;
mod analysis;
mod fields;

pub(crate) use analysis::BoundaryKind;
pub(crate) use analysis::{ParsedGardeFile, analyze, parse_rust_file};
