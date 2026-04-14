mod golden;
mod helpers;
mod missing;

mod wrong;

mod assertions {
    use guardrail3_check_types::G3CheckResult;

    use crate::test_support::{
        Finding, assert_findings_for, error_for, info_for,
    };

    const ID: &str = "RS-CLIPPY-CONFIG-05";

    pub(super) fn assert_findings(results: &[G3CheckResult], expected: &[Finding]) {
        assert_findings_for(results, ID, expected);
    }


    pub(super) fn error(title: &str, message: &str, file: &str, inventory: bool) -> Finding {
        error_for(ID, title, message, file, inventory)
    }


    pub(super) fn info(title: &str, message: &str, file: &str, inventory: bool) -> Finding {
        info_for(ID, title, message, file, inventory)
    }
}
