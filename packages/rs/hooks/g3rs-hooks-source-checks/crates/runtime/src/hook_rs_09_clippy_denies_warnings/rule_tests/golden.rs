use g3rs_hooks_source_checks_assertions::hook_rs_09_clippy_denies_warnings::rule as assertions;

use super::super::run_case;

#[test]
fn warns_for_clippy_without_deny_warnings() {
    let results = run_case("cargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_for_clippy_with_dash_d_warnings() {
    let results = run_case("cargo clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_for_clippy_with_compact_dash_dwarnings() {
    let results = run_case("cargo clippy --workspace -- -Dwarnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_for_clippy_with_deny_equals_warnings() {
    let results = run_case("cargo clippy --workspace -- --deny=warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_rustflags_denies_warnings() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_rustflags_denies_warnings() {
    let results = run_case("env RUSTFLAGS=-Dwarnings cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_rustflags_uses_deny_warnings_spelling() {
    let results = run_case("RUSTFLAGS='--deny warnings' cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_rustflags_uses_deny_equals_warnings() {
    let results = run_case("env RUSTFLAGS=--deny=warnings cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exported_rustflags_denies_warnings_before_clippy() {
    let results = run_case("export RUSTFLAGS='-D warnings'\ncargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_function_runs_clippy_with_deny_warnings() {
    let results =
        run_case("run_clippy() {\n    cargo clippy --workspace -- -D warnings\n}\nrun_clippy\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_clippy_with_deny_warnings() {
    let results = run_case("bash -lc 'cargo clippy --workspace -- -D warnings'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_uses_init_file_before_clippy_script() {
    let results =
        run_case("bash --init-file /tmp/bashrc -lc 'cargo clippy --workspace -- -D warnings'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_sh_option_value_precedes_clippy_script() {
    let results = run_case("sh -o errexit -c 'cargo clippy --workspace -- -D warnings'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_clippy_denies_warnings() {
    let results = run_case("cargo +nightly clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_clippy_denies_warnings() {
    let results = run_case("env -i cargo clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_jobs_short_flag_precedes_clippy() {
    let results = run_case("cargo -j 4 clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_jobs_attached_short_flag_precedes_clippy() {
    let results = run_case("cargo -j4 clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_chdir_precedes_clippy() {
    let results = run_case("cargo -C tools clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_clippy_denies_warnings() {
    let results = run_case("/Users/me/.cargo/bin/cargo clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_later_deny_overrides_earlier_allow_in_clippy_args() {
    let results = run_case("cargo clippy --workspace -- -A warnings -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_later_deny_overrides_earlier_warn_in_clippy_args() {
    let results = run_case("cargo clippy --workspace -- -W warnings -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_later_deny_overrides_earlier_allow_in_rustflags() {
    let results = run_case("RUSTFLAGS='-A warnings -D warnings' cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_clippy_args_override_softer_rustflags() {
    let results = run_case("RUSTFLAGS='-A warnings' cargo clippy --workspace -- -D warnings\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_rustflags_near_match_only_mentions_warnings() {
    let results = run_case("RUSTFLAGS='-D warnings_help' cargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_inline_rustflags_only_applies_to_other_command() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo fmt\ncargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_rustflags_only_applies_to_other_command() {
    let results = run_case("env RUSTFLAGS=-Dwarnings cargo fmt\ncargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_env_unset_only_applies_to_other_command() {
    let results = run_case(
        "export RUSTFLAGS='-D warnings'\nenv -u RUSTFLAGS cargo fmt\ncargo clippy --workspace\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_shell_wrapper_exports_rustflags_only_in_subshell() {
    let results =
        run_case("bash -lc 'export RUSTFLAGS=\"-D warnings\"'\ncargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_deny_warnings_only_appears_inside_echo() {
    let results = run_case("echo \"cargo clippy --workspace -- -D warnings\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_unsets_exported_rustflags_before_clippy() {
    let results =
        run_case("export RUSTFLAGS='-D warnings'\nenv -u RUSTFLAGS cargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_clippy_subcommand_is_used() {
    let results = run_case("cargo clippy-driver --workspace -- -D warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_command_is_only_help() {
    let results = run_case("cargo clippy --help -- -D warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_clippy() {
    let results = run_case("cargo --help clippy --workspace -- -D warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_cargo_command_mentions_clippy_text() {
    let results = run_case("cargo fmt --message-format 'cargo clippy -- -D warnings'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_later_allows_warnings_again() {
    let results = run_case("cargo clippy --workspace -- -D warnings -A warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_later_warns_warnings_again() {
    let results = run_case("cargo clippy --workspace -- -D warnings -W warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_force_warns_warnings() {
    let results = run_case("cargo clippy --workspace -- -D warnings --force-warn warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_caps_lints_to_allow() {
    let results = run_case("cargo clippy --workspace -- -D warnings --cap-lints allow\n");
    assertions::assert_missing(&results);
}
