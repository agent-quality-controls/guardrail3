use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_36_string_dispatch_cap::assert_no_hits;

#[test]
fn skips_small_sites_and_test_contexts() {
    let small_match =
        "pub fn dispatch(value: &str) -> usize { match value { \"a\" => 1, \"b\" => 2, _ => 0 } }";
    let test_only_dispatch = "#[test]\nfn dispatch_test() { let value = \"a\"; if value == \"a\" { } else if value == \"b\" { } else if value == \"c\" { } else if value == \"d\" { } else if value == \"e\" { } else if value == \"f\" { } else if value == \"g\" { } else if value == \"h\" { } else if value == \"i\" { } else if value == \"j\" { } else if value == \"k\" { } }";

    assert_no_hits(&check_source("src/lib.rs", small_match, false));
    assert_no_hits(&check_source("src/lib.rs", test_only_dispatch, false));
}
