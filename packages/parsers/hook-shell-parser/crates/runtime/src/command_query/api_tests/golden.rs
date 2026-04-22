use super::super::{
    CommandQueryOptions, CommandVisit, ShellEnvState, parse_script_for_tests,
    visit_resolved_commands_with_env,
};
use hook_shell_parser_runtime_assertions::command_query::api as query_assertions;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TestEnv {
    rustflags: Option<String>,
    target_dir: bool,
}

impl ShellEnvState for TestEnv {
    fn apply_assignment(&mut self, name: &str, value: &str) {
        match name {
            "RUSTFLAGS" => self.rustflags = Some(value.to_owned()),
            "CARGO_TARGET_DIR" => self.target_dir = true,
            _ => {}
        }
    }

    fn unset(&mut self, name: &str) {
        match name {
            "RUSTFLAGS" => self.rustflags = None,
            "CARGO_TARGET_DIR" => self.target_dir = false,
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.rustflags = None;
        self.target_dir = false;
    }
}

#[test]
fn resolves_called_function_with_path_qualified_command() {
    let script = "check_conflicts() {\n    /usr/bin/rg '<<<<<<<' .\n}\ncheck_conflicts\n";

    query_assertions::assert_script_has_resolved_command(script, "rg", "<<<<<<<");
}

#[test]
fn visits_commands_with_persisted_export_state() {
    let parsed = parse_script_for_tests("export RUSTFLAGS='-D warnings'\ncargo clippy\n");
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(
        snapshots,
        vec![("cargo".to_owned(), Some("-D warnings".to_owned()), false)]
    );
}

#[test]
fn visits_commands_with_env_wrapper_state_changes() {
    let parsed =
        parse_script_for_tests("export RUSTFLAGS='-D warnings'\nenv -u RUSTFLAGS cargo clippy\n");
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(snapshots, vec![("cargo".to_owned(), None, false)]);
}

#[test]
fn visits_function_calls_with_local_inline_env_without_persisting() {
    let parsed = parse_script_for_tests(
        "run_checks() {\n    cargo test\n}\nCARGO_TARGET_DIR=.cargo-target run_checks\ncargo test\n",
    );
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default().with_detached_commands(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(
        snapshots,
        vec![
            ("cargo".to_owned(), None, true),
            ("cargo".to_owned(), None, false),
        ]
    );
}

#[test]
fn env_ignore_environment_clears_shared_target_dir() {
    let parsed = parse_script_for_tests(
        "export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\nenv -i cargo test\n",
    );
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default().with_detached_commands(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(snapshots, vec![("cargo".to_owned(), None, false)]);
}

#[test]
fn visits_forward_called_functions_when_enabled() {
    let parsed =
        parse_script_for_tests("outer() {\n    inner\n}\ninner() {\n    cargo test\n}\nouter\n");
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default()
            .with_detached_commands()
            .with_forward_functions(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(snapshots, vec![("cargo".to_owned(), None, false)]);
}

#[test]
fn visits_backtick_command_substitutions() {
    let parsed = parse_script_for_tests(
        "export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\necho `cargo test`\n",
    );
    let mut snapshots = Vec::new();

    visit_resolved_commands_with_env(
        &parsed,
        TestEnv::default(),
        CommandQueryOptions::default().with_detached_commands(),
        |command, state| {
            snapshots.push((
                command.command_name().to_owned(),
                state.rustflags.clone(),
                state.target_dir,
            ));
            CommandVisit::Continue
        },
    );

    assert_eq!(
        snapshots,
        vec![
            ("echo".to_owned(), None, true),
            ("cargo".to_owned(), None, true)
        ]
    );
}
