pub fn assert_error(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_error(input, "g3ts-typecov/syncpack-type-coverage-pin", file);
}

pub fn assert_info(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_info(input, "g3ts-typecov/syncpack-type-coverage-pin", file);
}
