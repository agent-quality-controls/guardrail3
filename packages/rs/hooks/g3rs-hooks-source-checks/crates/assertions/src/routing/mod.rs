#![expect(
    clippy::module_name_repetitions,
    reason = "assertion helper requires direct indexing of expected fixed-size finding slice"
)]
pub mod discovers_marker_pair;
pub mod no_env_override_routing;
pub mod no_upward_walk_from_units;
pub mod scope_not_hardcoded_literal;
pub mod staged_files_diff_filter_acm;
