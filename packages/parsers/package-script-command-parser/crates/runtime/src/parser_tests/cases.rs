use package_script_command_parser_runtime_assertions::parser::{
    assert_command, assert_command_count, assert_eslint_invocation, assert_no_eslint_invocation,
    assert_parse_error_document, assert_parsed_document, assert_state_reason_contains,
    assert_unsupported_document, ExpectedEslintInvocation,
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
    assert_command(&document, 0, "pnpm eslint .", "pnpm", &["eslint", "."], None);
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
        (
            "lint:env",
            "env NODE_ENV=test eslint .",
            &["."][..],
        ),
        (
            "lint:cross-env",
            "cross-env NODE_ENV=test eslint .",
            &["."][..],
        ),
        (
            "lint:local-bin",
            "./node_modules/.bin/eslint .",
            &["."][..],
        ),
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
    let document = super::super::parse_document(
        "lint",
        "NODE_ENV=production CI=true eslint .",
    )
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
        ("lint:pattern-before-config", "eslint . --ignore-pattern --config eslint.config.mjs"),
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
fn unsupported_non_lint_script_without_eslint_is_not_a_lint_invocation() {
    let document = super::super::parse_document("build", "astro build | tee build.log")
        .expect("script command parser should produce a document");

    assert_no_eslint_invocation(&document);
}

#[test]
fn invalid_quote_in_lint_script_is_fail_closed_document_state() {
    let document = super::super::parse_document("lint", "eslint \"broken")
        .expect("script command parser should produce a document");

    assert_parse_error_document(&document);
    assert_state_reason_contains(&document, "unterminated quote");
}

#[test]
fn invalid_quote_after_exact_eslint_token_is_fail_closed_document_state() {
    let document = super::super::parse_document("check", "eslint \"broken")
        .expect("script command parser should produce a document");

    assert_parse_error_document(&document);
    assert_state_reason_contains(&document, "unterminated quote");
}
