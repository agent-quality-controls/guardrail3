use g3rs_code_ast_checks_types::G3RsCodeAstChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCodeAstChecksInput) -> Vec<G3CheckResult> {
    let parsed = match crate::support::parse_input(input) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            let parse_failure = crate::support::parse_failure_input(input, &parse_error);
            let mut results = Vec::new();
            crate::rs_code_ast_30_input_failures::check(&parse_failure, &mut results);
            return results;
        }
    };
    let rule_input = crate::support::CodeSourceRuleInput::from(&parsed);
    let mut results = Vec::new();

    crate::rs_code_ast_13_todo_macros::check(&rule_input, &mut results);
    crate::rs_code_ast_15_direct_fs_usage::check(&rule_input, &mut results);
    crate::rs_code_ast_16_panic_macro::check(&rule_input, &mut results);

    results
}
