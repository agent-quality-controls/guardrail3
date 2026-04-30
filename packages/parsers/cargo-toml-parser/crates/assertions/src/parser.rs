#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

pub use crate::parser_deps::{assert_detailed_dep_version, assert_lint_level, assert_simple_dep};
pub use crate::parser_manifest::{
    assert_manifest_empty, assert_package_name, assert_parse_error, assert_string_build_and_readme,
    assert_top_level_extra_string,
};
pub use crate::parser_realistic::{
    assert_alternative_known_multi_shape_fields, assert_realistic_manifest,
};
