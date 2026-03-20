use super::helpers::{arch_01_errors, copy_golden, run_check};

#[test]
fn golden_passes() {
    let tmp = copy_golden();
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "golden should have 0 RS-ARCH-01 errors, got: {errors:#?}");
}
