pub struct LintExpectation {
    pub name: &'static str,
    pub expected_level: &'static str,
    pub priority: Option<i64>,
}

pub const EXPECTED_RUST_LINTS: &[LintExpectation] = &[
    LintExpectation {
        name: "warnings",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unsafe_code",
        expected_level: "forbid",
        priority: None,
    },
    LintExpectation {
        name: "dead_code",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_results",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_crate_dependencies",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "missing_debug_implementations",
        expected_level: "warn",
        priority: None,
    },
];

pub const EXPECTED_LIBRARY_RUST_LINTS: &[LintExpectation] = &[LintExpectation {
    name: "unreachable_pub",
    expected_level: "deny",
    priority: None,
}];

pub const EXPECTED_CLIPPY_GROUPS: &[LintExpectation] = &[
    LintExpectation {
        name: "all",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "pedantic",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "cargo",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "nursery",
        expected_level: "deny",
        priority: Some(-1),
    },
];

pub const EXPECTED_CLIPPY_DENY: &[&str] = &[
    "unwrap_used",
    "expect_used",
    "panic",
    "unimplemented",
    "todo",
    "dbg_macro",
    "print_stdout",
    "print_stderr",
    "disallowed_methods",
    "disallowed_types",
    "indexing_slicing",
    "string_slice",
    "arithmetic_side_effects",
    "shadow_unrelated",
    "missing_assert_message",
    "partial_pub_fields",
    "str_to_string",
    "implicit_clone",
    "as_conversions",
    "float_cmp",
    "lossy_float_literal",
    "wildcard_enum_match_arm",
    "rest_pat_in_fully_bound_structs",
    "large_stack_arrays",
    "needless_pass_by_value",
    "redundant_else",
    "large_futures",
    "semicolon_if_nothing_returned",
    "redundant_closure_for_method_calls",
    "map_unwrap_or",
    "verbose_file_reads",
];

pub const EXPECTED_CLIPPY_ALLOW: &[&str] = &[
    "missing_docs_in_private_items",
    "module_name_repetitions",
    "must_use_candidate",
    "option_if_let_else",
    "empty_line_after_doc_comments",
    "single_match_else",
    "ref_option_ref",
    "trivially_copy_pass_by_ref",
    "multiple_crate_versions",
];

pub fn workspace_lints<'a>(parsed: &'a toml::Value, family: &str) -> Option<&'a toml::Value> {
    parsed
        .get("workspace")
        .and_then(|value| value.get("lints"))
        .and_then(|value| value.get(family))
}

pub fn member_lints<'a>(parsed: &'a toml::Value, family: &str) -> Option<&'a toml::Value> {
    parsed.get("lints").and_then(|value| value.get(family))
}

pub fn lint_level(lints: &toml::Value, name: &str) -> Option<String> {
    match lints.get(name) {
        Some(toml::Value::String(level)) => Some(level.clone()),
        Some(toml::Value::Table(table)) => table
            .get("level")
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

pub fn lint_priority(lints: &toml::Value, name: &str) -> Option<i64> {
    match lints.get(name) {
        Some(toml::Value::Table(table)) => table.get("priority").and_then(toml::Value::as_integer),
        _ => None,
    }
}

pub fn level_rank(level: &str) -> usize {
    match level {
        "allow" => 0,
        "warn" => 1,
        "deny" => 2,
        "forbid" => 3,
        _ => 0,
    }
}

pub fn is_weaker(expected_level: &str, actual_level: &str) -> bool {
    level_rank(actual_level) < level_rank(expected_level)
}
