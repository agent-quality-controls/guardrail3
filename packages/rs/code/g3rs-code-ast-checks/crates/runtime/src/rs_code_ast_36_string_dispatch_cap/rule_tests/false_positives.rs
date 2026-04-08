use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_36_string_dispatch_cap::assert_rule_results;

#[test]
fn skips_small_sites_and_test_contexts() {
    let small_match =
        "pub fn dispatch(value: &str) -> usize { match value { \"a\" => 1, \"b\" => 2, _ => 0 } }";
    let test_only_dispatch = "#[test]\nfn dispatch_test() { let value = \"a\"; if value == \"a\" { } else if value == \"b\" { } else if value == \"c\" { } else if value == \"d\" { } else if value == \"e\" { } else if value == \"f\" { } else if value == \"g\" { } else if value == \"h\" { } else if value == \"i\" { } else if value == \"j\" { } else if value == \"k\" { } }";
    let mixed_chain = "pub fn dispatch(a: &str, b: &str) { if a == \"a\" { } else if b == \"b\" { } else if a == \"c\" { } else if b == \"d\" { } else if a == \"e\" { } else if b == \"f\" { } else if a == \"g\" { } else if b == \"h\" { } else if a == \"i\" { } else if b == \"j\" { } else if a == \"k\" { } }";

    assert_rule_results(&check_source("src/lib.rs", small_match, false), &[]);
    assert_rule_results(&check_source("src/lib.rs", test_only_dispatch, false), &[]);
    assert_rule_results(&check_source("src/lib.rs", mixed_chain, false), &[]);
}
