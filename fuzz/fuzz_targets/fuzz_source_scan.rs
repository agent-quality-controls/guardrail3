#![no_main]
//! Fuzz target for guardrail3 Rust source scanning functions.
//!
//! Feeds arbitrary "Rust source" content to comment filtering,
//! allow checks, structure checks, and code quality checks.
//! Goal: none of these functions should panic on any input.

use libfuzzer_sys::fuzz_target;

use std::path::Path;

use guardrail3::app::rs::validate::allow_checks;
use guardrail3::app::rs::validate::code_quality_checks;
use guardrail3::app::rs::validate::source_scan;
use guardrail3::app::rs::validate::structure_checks;

fuzz_target!(|data: &[u8]| {
    // Only try valid UTF-8 — Rust source is text
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };

    // Use a synthetic path for all checks
    let path = Path::new("fuzz/test.rs");

    // 1. Comment filtering — core utility used by all source checks
    let _ = source_scan::filter_non_comment_lines(content);

    // 2. Allow checks (R30-R37)
    let mut results = Vec::new();
    allow_checks::check_crate_level_allow(path, content, false, false, &mut results);
    allow_checks::check_crate_level_allow(path, content, true, false, &mut results);
    allow_checks::check_crate_level_allow(path, content, false, true, &mut results);
    allow_checks::check_item_level_allow(path, content, &mut results);
    allow_checks::check_garde_skip(path, content, &mut results);
    allow_checks::check_cfg_attr_allow(path, content, &mut results);

    // 3. Structure checks (R38-R41)
    results.clear();
    structure_checks::check_file_length(path, content, false, &mut results);
    structure_checks::check_file_length(path, content, true, &mut results);
    structure_checks::check_use_count(path, content, false, &mut results);
    structure_checks::check_use_count(path, content, true, &mut results);

    // 4. Code quality checks (R43-R44, R58)
    results.clear();
    code_quality_checks::check_todo_macros(path, content, false, &mut results);
    code_quality_checks::check_todo_macros(path, content, true, &mut results);
    code_quality_checks::check_unwrap_expect(path, content, &mut results);
    code_quality_checks::check_direct_fs_usage(path, content, false, &mut results);
});
