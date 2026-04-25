use package_script_command_parser_runtime_assertions::parser::{
    ExpectedEslintInvocation, ExpectedToolInvocation, assert_command, assert_command_count,
    assert_eslint_invocation, assert_no_eslint_invocation, assert_parse_error_document,
    assert_parsed_document, assert_state_reason_contains, assert_tool_invocation,
    assert_unsupported_document,
};

use crate::types::PackageScriptCommandSeparator;

#[test]
fn parses_eslint_invocations_and_ignore_flags() {
    let document = super::super::parse_document(
        "lint",
        r"eslint . --ignore-pattern src/pages/** --ignore-pattern=src/app/** --ignore-path .eslintignore --config eslint.config.mjs",
    )
    .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_command_count(&document, 1);
    assert_command(
        &document,
        0,
        "eslint . --ignore-pattern src/pages/** --ignore-pattern=src/app/** --ignore-path .eslintignore --config eslint.config.mjs",
        "eslint",
        &[
            ".",
            "--ignore-pattern",
            "src/pages/**",
            "--ignore-pattern=src/app/**",
            "--ignore-path",
            ".eslintignore",
            "--config",
            "eslint.config.mjs",
        ],
        None,
    );
    assert_eslint_invocation(
        &document,
        0,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 0,
            invocation: "eslint . --ignore-pattern src/pages/** --ignore-pattern=src/app/** --ignore-path .eslintignore --config eslint.config.mjs",
            args: &[
                ".",
                "--ignore-pattern",
                "src/pages/**",
                "--ignore-pattern=src/app/**",
                "--ignore-path",
                ".eslintignore",
                "--config",
                "eslint.config.mjs",
            ],
            ignore_patterns: &["src/pages/**", "src/app/**"],
            ignore_path: Some(".eslintignore"),
            config_path: Some("eslint.config.mjs"),
        },
    );
}

#[test]
fn parses_pnpm_eslint_wrappers_and_separators() {
    let document = super::super::parse_document(
        "lint",
        "pnpm eslint . && pnpm exec eslint src --config=custom.config.mjs || echo done",
    )
    .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_command_count(&document, 3);
    assert_command(
        &document,
        0,
        "pnpm eslint .",
        "pnpm",
        &["eslint", "."],
        None,
    );
    assert_command(
        &document,
        1,
        "pnpm exec eslint src --config=custom.config.mjs",
        "pnpm",
        &["exec", "eslint", "src", "--config=custom.config.mjs"],
        Some(PackageScriptCommandSeparator::And),
    );
    assert_command(
        &document,
        2,
        "echo done",
        "echo",
        &["done"],
        Some(PackageScriptCommandSeparator::Or),
    );
    assert_eslint_invocation(
        &document,
        0,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 0,
            invocation: "pnpm eslint .",
            args: &["."],
            ignore_patterns: &[],
            ignore_path: None,
            config_path: None,
        },
    );
    assert_eslint_invocation(
        &document,
        1,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 1,
            invocation: "pnpm exec eslint src --config=custom.config.mjs",
            args: &["src", "--config=custom.config.mjs"],
            ignore_patterns: &[],
            ignore_path: None,
            config_path: Some("custom.config.mjs"),
        },
    );
}

#[test]
fn retains_commands_when_script_has_no_eslint_invocation() {
    let document = super::super::parse_document("check", "astro check && syncpack lint")
        .expect("script command document should parse");

    assert_no_eslint_invocation(&document);
    assert_command_count(&document, 2);
    assert_command(&document, 0, "astro check", "astro", &["check"], None);
    assert_command(
        &document,
        1,
        "syncpack lint",
        "syncpack",
        &["lint"],
        Some(PackageScriptCommandSeparator::And),
    );
    assert_tool_invocation(
        &document,
        0,
        ExpectedToolInvocation {
            script_name: "check",
            command_index: 0,
            invocation: "astro check",
            executable: "astro",
            args: &["check"],
            preceded_by: None,
            followed_by: Some(PackageScriptCommandSeparator::And),
        },
    );
    assert_tool_invocation(
        &document,
        1,
        ExpectedToolInvocation {
            script_name: "check",
            command_index: 1,
            invocation: "syncpack lint",
            executable: "syncpack",
            args: &["lint"],
            preceded_by: Some(PackageScriptCommandSeparator::And),
            followed_by: None,
        },
    );
}

#[test]
fn normalizes_non_eslint_package_runner_invocations() {
    let document = super::super::parse_document(
        "check",
        "npx --yes astro check && pnpm --filter app exec syncpack lint",
    )
    .expect("script command document should parse");

    assert_no_eslint_invocation(&document);
    assert_tool_invocation(
        &document,
        0,
        ExpectedToolInvocation {
            script_name: "check",
            command_index: 0,
            invocation: "npx --yes astro check",
            executable: "astro",
            args: &["check"],
            preceded_by: None,
            followed_by: Some(PackageScriptCommandSeparator::And),
        },
    );
    assert_tool_invocation(
        &document,
        1,
        ExpectedToolInvocation {
            script_name: "check",
            command_index: 1,
            invocation: "pnpm --filter app exec syncpack lint",
            executable: "syncpack",
            args: &["lint"],
            preceded_by: Some(PackageScriptCommandSeparator::And),
            followed_by: None,
        },
    );
}

#[test]
fn safe_tool_invocation_query_rejects_fail_open_or_chains() {
    let safe = super::super::parse("check", "astro check && syncpack lint")
        .expect("script command fact should parse");
    let fake_text = super::super::parse("check", "echo syncpack lint")
        .expect("script command fact should parse");
    let unsafe_after = super::super::parse("check", "astro check && syncpack lint || true")
        .expect("script command fact should parse");
    let unsafe_before = super::super::parse("check", "true || syncpack lint")
        .expect("script command fact should parse");
    let unsafe_later_or = super::super::parse("check", "syncpack lint && true || true")
        .expect("script command fact should parse");
    let unsafe_astro_later_or = super::super::parse("check", "astro check && true || true")
        .expect("script command fact should parse");
    let unsafe_newline_chain = super::super::parse("check", "astro check\ntrue")
        .expect("script command fact should parse");
    let unsafe_background_build = super::super::parse("build", "astro build &")
        .expect("script command fact should parse");
    let unsafe_duplicate_surface = super::super::parse("test", "syncpack lint || true")
        .expect("script command fact should parse");
    let fake_only_allow = super::super::parse("preinstall", "echo only-allow pnpm")
        .expect("script command fact should parse");
    let unsafe_only_allow = super::super::parse("preinstall", "only-allow pnpm || true")
        .expect("script command fact should parse");
    let unsupported_only_allow = super::super::parse("preinstall", "only-allow pnpm | tee log")
        .expect("script command fact should parse");

    assert!(
        super::super::has_safe_tool_invocation(std::slice::from_ref(&safe), "syncpack", "lint"),
        "syncpack lint in an && chain should be accepted"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[fake_text], "syncpack", "lint"),
        "fake syncpack lint text should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_after], "syncpack", "lint"),
        "syncpack lint followed by || should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_before], "syncpack", "lint"),
        "syncpack lint preceded by || should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_later_or], "syncpack", "lint"),
        "syncpack lint in a later fail-open || chain should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_astro_later_or], "astro", "check"),
        "astro check in a later fail-open || chain should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_newline_chain], "astro", "check"),
        "newline-separated astro check scripts should be rejected as unsupported shell syntax"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_background_build], "astro", "build"),
        "backgrounded astro build should be rejected as unsupported shell syntax"
    );
    assert!(
        !super::super::has_safe_tool_invocation(
            &[safe, unsafe_duplicate_surface],
            "syncpack",
            "lint"
        ),
        "one safe syncpack lint script must not mask another fail-open syncpack lint script"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[fake_only_allow], "only-allow", "pnpm"),
        "echoed only-allow text should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsafe_only_allow], "only-allow", "pnpm"),
        "fail-open only-allow invocation should be rejected"
    );
    assert!(
        !super::super::has_safe_tool_invocation(&[unsupported_only_allow], "only-allow", "pnpm"),
        "unsupported only-allow shell syntax should be rejected"
    );
}

#[test]
fn unsupported_guardrail_shell_syntax_fails_closed() {
    for command in [
        "syncpack lint | tee log",
        "astro check; syncpack lint",
        "astro build &",
        "echo $(syncpack lint)",
    ] {
        let document = super::super::parse_document("check", command)
            .expect("script command parser should produce a document");

        assert_unsupported_document(&document);
        assert_state_reason_contains(&document, "unsupported shell syntax");
    }
}

#[test]
fn parses_wrapper_flags_before_eslint_executables() {
    let document = super::super::parse_document(
        "lint",
        "npx --yes eslint . --ignore-path .eslintignore && pnpm --filter app exec eslint . --ignore-pattern src/content/**",
    )
    .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_eslint_invocation(
        &document,
        0,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 0,
            invocation: "npx --yes eslint . --ignore-path .eslintignore",
            args: &[".", "--ignore-path", ".eslintignore"],
            ignore_patterns: &[],
            ignore_path: Some(".eslintignore"),
            config_path: None,
        },
    );
    assert_eslint_invocation(
        &document,
        1,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 1,
            invocation: "pnpm --filter app exec eslint . --ignore-pattern src/content/**",
            args: &[".", "--ignore-pattern", "src/content/**"],
            ignore_patterns: &["src/content/**"],
            ignore_path: None,
            config_path: None,
        },
    );
}

#[test]
fn parses_common_eslint_wrappers() {
    for (script, command, expected_args) in [
        (
            "lint:npm",
            "npm exec eslint . --config eslint.config.mjs",
            &[".", "--config", "eslint.config.mjs"][..],
        ),
        (
            "lint:npm-double-dash",
            "npm exec -- eslint . --ignore-path .eslintignore",
            &[".", "--ignore-path", ".eslintignore"][..],
        ),
        ("lint:yarn", "yarn eslint src", &["src"][..]),
        ("lint:bun", "bun eslint src", &["src"][..]),
        ("lint:bunx", "bunx eslint src", &["src"][..]),
        ("lint:env", "env NODE_ENV=test eslint .", &["."][..]),
        (
            "lint:cross-env",
            "cross-env NODE_ENV=test eslint .",
            &["."][..],
        ),
        ("lint:local-bin", "./node_modules/.bin/eslint .", &["."][..]),
    ] {
        let document = super::super::parse_document(script, command)
            .expect("script command document should parse");

        assert_parsed_document(&document);
        assert_eslint_invocation(
            &document,
            0,
            ExpectedEslintInvocation {
                script_name: script,
                command_index: 0,
                invocation: command,
                args: expected_args,
                ignore_patterns: &[],
                ignore_path: if command.contains("--ignore-path") {
                    Some(".eslintignore")
                } else {
                    None
                },
                config_path: if command.contains("--config") {
                    Some("eslint.config.mjs")
                } else {
                    None
                },
            },
        );
    }
}

#[test]
fn package_manager_install_commands_are_not_eslint_invocations() {
    for command in ["pnpm add eslint", "pnpm why eslint", "npm install eslint"] {
        let document = super::super::parse_document("deps", command)
            .expect("script command document should parse");

        assert_no_eslint_invocation(&document);
    }
}

#[test]
fn strips_environment_assignments_before_executable() {
    let document = super::super::parse_document("lint", "NODE_ENV=production CI=true eslint .")
        .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_command(
        &document,
        0,
        "NODE_ENV=production CI=true eslint .",
        "eslint",
        &["."],
        None,
    );
}

#[test]
fn empty_and_lowercase_environment_assignments_do_not_hide_eslint() {
    let document = super::super::parse_document("lint", "ci= CI= eslint .")
        .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_command(&document, 0, "ci= CI= eslint .", "eslint", &["."], None);
}

#[test]
fn no_eslint_invocation_is_explicit_state() {
    let document = super::super::parse_document("build", "astro build")
        .expect("script command document should parse");

    assert_no_eslint_invocation(&document);
}

#[test]
fn missing_eslint_flag_values_fail_closed() {
    for (script, command) in [
        ("lint:ignore-pattern", "eslint . --ignore-pattern"),
        ("lint:ignore-pattern-attached", "eslint . --ignore-pattern="),
        ("lint:ignore-path", "eslint . --ignore-path"),
        ("lint:ignore-path-attached", "eslint . --ignore-path="),
        ("lint:config-long", "eslint . --config"),
        ("lint:config-long-attached", "eslint . --config="),
        ("lint:config-short", "eslint . -c"),
        (
            "lint:pattern-before-config",
            "eslint . --ignore-pattern --config eslint.config.mjs",
        ),
    ] {
        let document = super::super::parse_document(script, command)
            .expect("script command parser should produce a document");

        assert_parse_error_document(&document);
        assert_state_reason_contains(&document, "missing a value");
    }
}

#[test]
fn quoted_flag_values_are_parsed_and_dash_dash_stops_option_scanning() {
    let document = super::super::parse_document(
        "lint",
        "eslint . --ignore-pattern \"src/content/**\" --config 'eslint.config.mjs' -- --ignore-path hidden",
    )
    .expect("script command document should parse");

    assert_parsed_document(&document);
    assert_eslint_invocation(
        &document,
        0,
        ExpectedEslintInvocation {
            script_name: "lint",
            command_index: 0,
            invocation: "eslint . --ignore-pattern \"src/content/**\" --config 'eslint.config.mjs' -- --ignore-path hidden",
            args: &[
                ".",
                "--ignore-pattern",
                "src/content/**",
                "--config",
                "eslint.config.mjs",
                "--",
                "--ignore-path",
                "hidden",
            ],
            ignore_patterns: &["src/content/**"],
            ignore_path: None,
            config_path: Some("eslint.config.mjs"),
        },
    );
}

#[test]
fn exact_eslint_command_in_non_lint_script_is_lint_related() {
    let document = super::super::parse_document("check", "eslint . | tee lint.log")
        .expect("script command parser should produce a document");

    assert_unsupported_document(&document);
    assert_state_reason_contains(&document, "unsupported");
}

#[test]
fn exact_eslint_command_adjacent_to_operator_fails_closed() {
    let document = super::super::parse_document("check", "eslint|tee lint.log")
        .expect("script command parser should produce a document");

    assert_unsupported_document(&document);
    assert_state_reason_contains(&document, "unsupported");
}

#[test]
fn shell_expansion_in_lint_scripts_fails_closed() {
    for command in [
        r#"eslint . --ignore-pattern "$ASTRO_IGNORE_GLOB""#,
        "eslint . --ignore-pattern `node hidden-ignore.js`",
    ] {
        let document = super::super::parse_document("lint", command)
            .expect("script command parser should produce a document");

        assert_unsupported_document(&document);
        assert_state_reason_contains(&document, "unsupported");
    }
}

#[test]
fn unsupported_background_operator_fails_closed() {
    let document = super::super::parse_document("lint", "eslint . & echo done")
        .expect("script command parser should produce a document");

    assert_unsupported_document(&document);
    assert_state_reason_contains(&document, "unsupported");
}

#[test]
fn unsupported_lint_related_separator_fails_closed() {
    let document = super::super::parse_document("lint", "eslint . | tee lint.log")
        .expect("script command parser should produce a document");

    assert_unsupported_document(&document);
    assert_state_reason_contains(&document, "unsupported");
}

#[test]
fn unsupported_guardrail_script_without_eslint_fails_closed() {
    let document = super::super::parse_document("build", "astro build | tee build.log")
        .expect("script command parser should produce a document");

    assert_unsupported_document(&document);
}

#[test]
fn invalid_quote_in_lint_script_is_fail_closed_document_state() {
    let document = super::super::parse_document("lint", "eslint \"broken")
        .expect("script command parser should produce a document");

    assert_parse_error_document(&document);
    assert_state_reason_contains(&document, "invalid shell syntax");
}

#[test]
fn invalid_quote_after_exact_eslint_token_is_fail_closed_document_state() {
    let document = super::super::parse_document("check", "eslint \"broken")
        .expect("script command parser should produce a document");

    assert_parse_error_document(&document);
    assert_state_reason_contains(&document, "invalid shell syntax");
}
