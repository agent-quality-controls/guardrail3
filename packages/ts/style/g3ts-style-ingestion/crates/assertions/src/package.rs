/// Assert that `invocation` has the expected executable and args.
///
/// # Panics
///
/// Panics when `invocation.executable` differs from `expected_executable`,
/// or when `invocation.args` differs from `expected_args`.
pub fn assert_tool_invocation(
    invocation: &g3ts_style_types::G3TsStylePackageScriptToolInvocation,
    expected_executable: &str,
    expected_args: &[&str],
) {
    assert_eq!(
        invocation.executable, expected_executable,
        "package script invocation should use expected executable"
    );
    assert_eq!(
        invocation.args,
        expected_args
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect::<Vec<_>>(),
        "package script invocation should use expected args"
    );
}
