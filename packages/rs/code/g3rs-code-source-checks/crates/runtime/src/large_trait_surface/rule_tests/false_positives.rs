#![allow(
    clippy::format_collect,
    clippy::format_in_format_args,
    reason = "test fixtures synthesize large source bodies via format! over an iterator; the simpler iterator-collect form is intentional"
)]

use g3rs_code_source_checks_assertions::large_trait_surface::rule::assert_rule_results;

#[test]
fn skips_trait_at_warn_boundary() {
    let methods = (0..8)
        .map(|index| format!("    fn m{index}(&self);\n"))
        .collect::<String>();
    let content = format!("pub trait Service {{\n{methods}}}");

    assert_rule_results(
        &super::super::check_source("src/lib.rs", &content, false),
        &[],
    );
}
