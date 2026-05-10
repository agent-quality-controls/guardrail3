/// Asserts the runtime emits an error for `syncpack-type-coverage-pin` at `file`.
pub fn assert_error(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_error(input, "g3ts-typecov/syncpack-type-coverage-pin", file);
}

/// Asserts the runtime emits an info for `syncpack-type-coverage-pin` at `file`.
pub fn assert_info(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_info(input, "g3ts-typecov/syncpack-type-coverage-pin", file);
}
