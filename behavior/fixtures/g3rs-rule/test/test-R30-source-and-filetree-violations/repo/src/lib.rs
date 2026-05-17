pub use std::fmt::Debug;

#[cfg(test)]
mod tests {
    #[test]
    fn inline_test_body_is_project_policy_violation() {
        let observed = [1_u8].len();
        assert!(
            observed == 1,
            "fixture assertion must inspect runtime state"
        );
    }

    #[ignore]
    #[test]
    fn ignored_test_without_reason_is_project_policy_violation() {
        let observed = ["fixture"].join("");
        assert_eq!(
            observed.as_str(),
            "fixture",
            "ignored fixture assertion must inspect runtime state"
        );
    }

    #[should_panic]
    #[test]
    fn should_panic_without_expected_is_project_policy_violation() {
        std::panic::panic_any("fixture panic");
    }

    #[test]
    fn missing_real_proof_is_project_policy_violation() {
        let _observed = String::from("fixture").len();
    }

    #[test]
    fn weak_expect_message_is_project_policy_violation() {
        let value = [1_u8].first();
        let _observed = value.expect("ok");
    }

    #[test]
    fn indirect_expect_message_is_project_policy_violation() {
        let value = [1_u8].first();
        let message = "fixture expect message should be literal";
        let _observed = value.expect(message);
    }
}
