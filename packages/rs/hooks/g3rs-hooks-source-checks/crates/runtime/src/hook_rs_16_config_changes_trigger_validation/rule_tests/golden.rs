use g3rs_hooks_source_checks_assertions::hook_rs_16_config_changes_trigger_validation::rule as assertions;

use super::super::run_case;

#[test]
fn warns_when_config_names_only_appear_in_comment() {
    let content = "# guardrail3-rs.toml clippy.toml .clippy.toml deny.toml .deny.toml rustfmt.toml .rustfmt.toml rust-toolchain.toml\n";
    let results = run_case(content);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some(
                "incomplete Rust guardrail config trigger coverage in `.githooks/pre-commit`",
            ),
            message_contains: Some("guardrail3-rs.toml"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_trigger_logic_checks_guardrail3_rs_toml() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_direct_grep_and_validation_chain_covers_all_configs() {
    let content = r#"echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$' && g3rs validate --path ."#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_direct_trigger_line_calls_helper_defined_elsewhere() {
    let content = r#"
run_guardrails() {
    g3rs validate --path .
}

echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$' && run_guardrails
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_trigger_logic_checks_all_rust_guardrail_configs() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_config_names_only_appear_in_echo_banner() {
    let content = r#"echo "$STAGED_FILES guardrail3-rs.toml clippy.toml .clippy.toml deny.toml .deny.toml rustfmt.toml .rustfmt.toml rust-toolchain.toml""#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_printf_prose_mentions_configs_before_validation() {
    let content = r#"
if printf '%s\n' 'guardrail3-rs.toml clippy.toml .clippy.toml deny.toml .deny.toml rustfmt.toml .rustfmt.toml rust-toolchain.toml'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_heredoc_mentions_configs_before_validation() {
    let content = r#"
cat <<'EOF'
guardrail3-rs.toml
clippy.toml
.clippy.toml
deny.toml
.deny.toml
rustfmt.toml
.rustfmt.toml
rust-toolchain.toml
EOF
g3rs validate --path .
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_doc_grep_mentions_configs_before_validation() {
    let content = r#"grep -q 'guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml' README.md && g3rs validate --path ."#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_config_trigger_block_does_not_run_guardrail_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    echo "config changed"
fi

if echo "$STAGED_FILES" | grep -qE '(\.rs|Cargo\.toml)$'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_only_dotted_variants_are_covered() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(\.clippy\.toml|\.deny\.toml|\.rustfmt\.toml|guardrail3-rs\.toml|rust-toolchain\.toml)$'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_config_trigger_block_runs_path_qualified_guardrail_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    /usr/local/bin/g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_all_configs_are_checked_in_a_single_line_if_block() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then g3rs validate --path .; fi"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_only_lookalike_config_names_are_covered() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(myguardrail3-rs\.toml|custom-clippy\.toml|project\.clippy\.toml|company-deny\.toml|project\.deny\.toml|team-rustfmt\.toml|project\.rustfmt\.toml|rust-toolchain\.toml\.bak)$'; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_case_trigger_covers_all_rust_guardrail_configs() {
    let content = r#"
case "$changed_path" in
    guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)
        g3rs validate --path .
        ;;
esac
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_if_branch_mentions_configs_but_else_branch_runs_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    echo "config changed"
else
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_case_branch_mentions_configs_but_other_branch_runs_validation() {
    let content = r#"
case "$changed_path" in
    guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)
        echo "config changed"
        ;;
    *.rs)
        g3rs validate --path .
        ;;
esac
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_single_line_if_mentions_configs_but_else_branch_runs_validation() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then echo "config changed"; else g3rs validate --path .; fi"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_discarded_trigger_comparison_does_not_guard_validation() {
    let content = r#"if true; then [[ "$changed_path" == *guardrail3-rs.toml* ]]; g3rs validate --path .; fi"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_compact_single_line_if_routes_config_changes_to_validation() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$';then g3rs validate --path .;fi"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_nested_if_and_case_route_config_changes_to_validation() {
    let content = r#"
if test -n "$changed_path"; then
    case "$changed_path" in
        guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)
            g3rs validate --path .
            ;;
    esac
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_matching_branch_calls_helper_defined_outside_branch() {
    let content = r#"
run_guardrails() {
    g3rs validate --path .
}

if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    run_guardrails
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_helper_branch_trigger_reaches_validation() {
    let content = r#"
should_validate_configs() {
    echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'
}

if test -n "$changed_path"; then
    should_validate_configs
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_nested_case_mentions_configs_but_other_arm_runs_validation() {
    let content = r#"
if test -n "$changed_path"; then
    case "$changed_path" in
        guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)
            echo "config changed"
            ;;
        *.rs)
            g3rs validate --path .
            ;;
    esac
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_elif_trigger_routes_config_changes_to_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(\.rs|Cargo\.toml)$'; then
    echo "rust files changed"
elif [[ "$STAGED_FILES" == *guardrail3-rs.toml* || "$STAGED_FILES" == *clippy.toml* || "$STAGED_FILES" == *.clippy.toml* || "$STAGED_FILES" == *deny.toml* || "$STAGED_FILES" == *.deny.toml* || "$STAGED_FILES" == *rustfmt.toml* || "$STAGED_FILES" == *.rustfmt.toml* || "$STAGED_FILES" == *rust-toolchain.toml* ]]; then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_multiline_if_condition_lists_configs_before_then() {
    let content = r#"
if [ "$changed" = "guardrail3-rs.toml" ] || \
   [ "$changed" = "clippy.toml" ] || \
   [ "$changed" = ".clippy.toml" ] || \
   [ "$changed" = "deny.toml" ] || \
   [ "$changed" = ".deny.toml" ] || \
   [ "$changed" = "rustfmt.toml" ] || \
   [ "$changed" = ".rustfmt.toml" ] || \
   [ "$changed" = "rust-toolchain.toml" ]
then
    g3rs validate --path .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_single_line_case_routes_config_changes_to_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml) g3rs validate --path . ;; esac"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_single_line_case_mentions_configs_but_other_arm_runs_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml) echo "config changed" ;; *.rs) g3rs validate --path . ;; esac"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_compact_single_line_case_routes_config_changes_to_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)g3rs validate --path .;;esac"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}
