pub fn assert_error(input: &g3ts_fmt_types::G3TsFmtConfigChecksInput, file: Option<&str>) {
    crate::run::assert_runtime_error(input, "g3ts-fmt/policy-configured", file);
}
