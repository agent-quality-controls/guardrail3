/// Asserts the runtime emits an error for `package-present` at `file`.
pub fn assert_error(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_error(input, "g3ts-typecov/package-present", file);
}
