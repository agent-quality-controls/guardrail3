mod macros;
mod methods;
mod render;
mod settings;
mod thresholds;
mod types;

pub use macros::{EXPECTED_MACRO_BANS, MACRO_DEBUGGING};
pub use methods::{
    METHOD_BLOCKING_SLEEP, METHOD_ENV_MUTATION, METHOD_ENV_VARS, METHOD_FILESYSTEM,
    METHOD_GARDE_DESERIALIZATION, METHOD_HTTP_CLIENT, METHOD_PROCESS_CONTROL, SERVICE_METHOD_PATHS,
    library_profile_methods, service_profile_methods,
};
pub use render::build_clippy_toml;
pub use settings::{
    ALLOW_DBG_IN_TESTS, ALLOW_PRINT_IN_TESTS, AVOID_BREAKING_EXPORTED_API, SETTINGS,
};
pub use thresholds::{
    COGNITIVE_COMPLEXITY_THRESHOLD, EXCESSIVE_NESTING_THRESHOLD, MAX_FN_PARAMS_BOOLS,
    MAX_STRUCT_BOOLS, THRESHOLD_VALUES, THRESHOLDS, TOO_MANY_ARGUMENTS_THRESHOLD,
    TOO_MANY_LINES_THRESHOLD, TYPE_COMPLEXITY_THRESHOLD,
};
pub use types::{
    BASE_TYPE_PATHS, LIBRARY_EXTRA_TYPE_PATHS, TYPE_COLLECTIONS, TYPE_DYNAMIC, TYPE_FILESYSTEM,
    TYPE_GARDE_EXTRACTORS, TYPE_GLOBAL_STATE, TYPE_SYNC, library_profile_types,
    pure_layer_extra_types, service_profile_types,
};

#[cfg(test)]
#[path = "clippy_tests.rs"]
mod tests;
