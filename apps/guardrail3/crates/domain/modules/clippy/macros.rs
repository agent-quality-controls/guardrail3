use crate::domain::modules::Module;

pub const EXPECTED_MACRO_BANS: &[&str] = &["println", "eprintln", "dbg", "todo", "unimplemented"];

pub const MACRO_DEBUGGING: Module = Module {
    name: "clippy/macros/debugging",
    description: "Ban debugging and unfinished-code macros",
    content: r#"    { path = "println", reason = "Use structured logging or explicit output helpers instead of ad hoc println!" },
    { path = "eprintln", reason = "Use structured logging or explicit error reporting instead of ad hoc eprintln!" },
    { path = "dbg", reason = "Remove debugging probes before commit -- dbg! is a temporary local diagnostic only" },
    { path = "todo", reason = "Ship concrete code or explicit errors -- todo! leaves unfinished control flow in production code" },
    { path = "unimplemented", reason = "Ship concrete code or explicit errors -- unimplemented! leaves unfinished control flow in production code" },"#,
};
