// This file starts with valid Rust but has a syntax error halfway through.
// guardrail3 must handle partial files gracefully — don't crash, don't skip
// the valid portion. grep doesn't care (it's line-by-line). AST parsers
// may fail entirely or recover partially.

use std::collections::BTreeMap;

#[allow(dead_code)] // reason: valid code before syntax error
fn valid_function() -> i32 {
    let map = BTreeMap::new();
    let _ = map;
    42
}

#[allow(unused)] // reason: another valid allow before the error
fn also_valid() {
    let x = "hello";
    let _ = x;
}

// === SYNTAX ERROR BELOW THIS LINE ===

fn broken_function( {
    // Missing closing paren and has an extra brace
    let y = 42
    // Missing semicolon too
}

// This function is valid syntax but comes AFTER the error.
// Can the parser recover and see this?
#[allow(dead_code)] // reason: post-error allow — can parser see this?
fn post_error_function() -> i32 {
    99
}
