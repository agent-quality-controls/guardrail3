/// Rejects half-adopted TS marker pairs in the pre-commit hook.
pub(crate) mod discovers_marker_pair;
/// Forbids env-variable-driven routing in the pre-commit hook.
pub(crate) mod no_env_override_routing;
/// Forbids ancestor walks over discovered owning TS units.
pub(crate) mod no_upward_walk_from_units;
/// Forbids hardcoded scope literals passed to the per-package validator.
pub(crate) mod scope_not_hardcoded_literal;
/// Requires `--diff-filter=ACM` on staged-file collection.
pub(crate) mod staged_files_diff_filter_acm;
/// Shared helpers used by routing rules to inspect parsed shell scripts.
pub(crate) mod support;
