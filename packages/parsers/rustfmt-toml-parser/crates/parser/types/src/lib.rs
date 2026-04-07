/// Typed rustfmt.toml model definitions.
mod rustfmt_toml;

pub use rustfmt_toml::{
    BraceStyle, Color, ControlBraceStyle, Edition, EmitMode, FloatLiteralTrailingZero,
    GroupImportsTactic, Heuristics, HexLiteralCase, ImportGranularity, IndentStyle,
    MatchArmLeadingPipe, NewlineStyle, RustfmtToml, StyleEdition, Version,
};
