use super::helpers::{arch_errors, copy_fixture, run_check};

#[test]
fn golden_passes() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.is_empty(),
        "golden should have 0 RS-ARCH-01 errors, got: {errors:#?}"
    );
}
