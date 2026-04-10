use g3rs_hooks_rs_source_checks_assertions::hook_rs_16_config_changes_trigger_validation as assertions;

use crate::hook_rs_16_config_changes_trigger_validation::run_case;

#[test]
fn warns_when_config_names_only_appear_in_comment() {
    let content = "# guardrail3-rs.toml clippy.toml .clippy.toml deny.toml .deny.toml rustfmt.toml .rustfmt.toml rust-toolchain.toml\n";
    let results = run_case(content);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("Rust config-change trigger coverage incomplete"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_trigger_logic_checks_guardrail3_rs_toml() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    guardrail3 rs validate --staged .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_trigger_logic_checks_all_rust_guardrail_configs() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    guardrail3 rs validate --staged .
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
fn warns_when_config_trigger_block_does_not_run_guardrail_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    echo "config changed"
fi

if echo "$STAGED_FILES" | grep -qE '(\.rs|Cargo\.toml)$'; then
    guardrail3 rs validate --staged .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_only_dotted_variants_are_covered() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(\.clippy\.toml|\.deny\.toml|\.rustfmt\.toml|guardrail3-rs\.toml|rust-toolchain\.toml)$'; then
    guardrail3 rs validate --staged .
fi
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_config_trigger_block_runs_path_qualified_guardrail_validation() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    /usr/local/bin/guardrail3 rs validate --staged .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_all_configs_are_checked_in_a_single_line_if_block() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then guardrail3 rs validate --staged .; fi"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_only_lookalike_config_names_are_covered() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(myguardrail3-rs\.toml|custom-clippy\.toml|project\.clippy\.toml|company-deny\.toml|project\.deny\.toml|team-rustfmt\.toml|project\.rustfmt\.toml|rust-toolchain\.toml\.bak)$'; then
    guardrail3 rs validate --staged .
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
        guardrail3 rs validate --staged .
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
    guardrail3 rs validate --staged .
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
        guardrail3 rs validate --staged .
        ;;
esac
"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_single_line_if_mentions_configs_but_else_branch_runs_validation() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then echo "config changed"; else guardrail3 rs validate --staged .; fi"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_compact_single_line_if_routes_config_changes_to_validation() {
    let content = r#"if echo "$STAGED_FILES" | grep -qE '(guardrail3-rs\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$';then guardrail3 rs validate --staged .;fi"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_nested_if_and_case_route_config_changes_to_validation() {
    let content = r#"
if test -n "$changed_path"; then
    case "$changed_path" in
        guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)
            guardrail3 rs validate --staged .
            ;;
    esac
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
            guardrail3 rs validate --staged .
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
    guardrail3 rs validate --staged .
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
    guardrail3 rs validate --staged .
fi
"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn passes_when_single_line_case_routes_config_changes_to_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml) guardrail3 rs validate --staged . ;; esac"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}

#[test]
fn warns_when_single_line_case_mentions_configs_but_other_arm_runs_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml) echo "config changed" ;; *.rs) guardrail3 rs validate --staged . ;; esac"#;
    let results = run_case(content);
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_compact_single_line_case_routes_config_changes_to_validation() {
    let content = r#"case "$changed_path" in guardrail3-rs.toml|clippy.toml|.clippy.toml|deny.toml|.deny.toml|rustfmt.toml|.rustfmt.toml|rust-toolchain.toml)guardrail3 rs validate --staged .;;esac"#;
    let results = run_case(content);
    assertions::assert_present(&results);
}
