pub const AVOID_BREAKING_EXPORTED_API: bool = false;
pub const ALLOW_DBG_IN_TESTS: bool = false;
pub const ALLOW_PRINT_IN_TESTS: bool = false;

pub const SETTINGS: &str = r"# Keep lint signal on exported APIs instead of suppressing compatibility-sensitive checks.
avoid-breaking-exported-api = false

# Tests should stay quiet and deterministic.
allow-dbg-in-tests = false
allow-print-in-tests = false";
