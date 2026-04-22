#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailOpenWrapper {
    True,
    NoOp,
    Echo(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutableLine {
    pub line_no: usize,
    pub raw: String,
    pub command_text: String,
    pub command_name: String,
    pub softened_by: Option<FailOpenWrapper>,
    pub is_dispatcher_syntax: bool,
    pub is_exit_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLine {
    pub line_no: usize,
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedShellScript {
    pub shebang: Option<String>,
    pub source_lines: Vec<SourceLine>,
    pub executable_lines: Vec<ExecutableLine>,
    pub functions: Vec<ShellFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFunction {
    pub name: String,
    pub line_no: usize,
    pub body: String,
    pub body_starts_on_definition_line: bool,
    pub parsed_body: Box<ParsedShellScript>,
}
