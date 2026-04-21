use hook_shell_parser::command_query::ShellEnvState;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct EnvState {
    pub(super) target_dir: bool,
}

impl ShellEnvState for EnvState {
    fn apply_assignment(&mut self, name: &str, _value: &str) {
        if name == "CARGO_TARGET_DIR" {
            self.target_dir = true;
        }
    }

    fn unset(&mut self, name: &str) {
        if name == "CARGO_TARGET_DIR" {
            self.target_dir = false;
        }
    }

    fn clear(&mut self) {
        self.target_dir = false;
    }
}
