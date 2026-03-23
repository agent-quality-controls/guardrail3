use crate::app::rs::checks::hooks::shell::ParsedShellScript;

pub struct RustHookCommandInput<'a> {
    pub rel_path: &'a str,
    pub parsed: &'a ParsedShellScript<'a>,
}
