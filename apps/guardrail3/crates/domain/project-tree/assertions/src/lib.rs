use guardrail3_domain_project_tree as _;

pub fn assert_string_vec_eq(actual: &[String], expected: &[&str], context: &str) {
    let expected = expected.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    assert_eq!(actual, expected, "{context}");
}
