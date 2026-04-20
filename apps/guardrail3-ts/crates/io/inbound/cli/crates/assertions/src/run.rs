/// Checks the final CLI output payload.
///
/// # Panics
///
/// Panics if the actual CLI output does not match the expected stdout, stderr, or exit code.
pub fn assert_cli_output(
    stdout: &str,
    stderr: &str,
    exit_code: i32,
    expected_stdout: &str,
    expected_stderr: &str,
    expected_exit_code: i32,
) {
    assert_eq!(
        stdout, expected_stdout,
        "stdout should match expected output"
    );
    assert_eq!(
        stderr, expected_stderr,
        "stderr should match expected output"
    );
    assert_eq!(
        exit_code, expected_exit_code,
        "exit code should match expected output"
    );
}
