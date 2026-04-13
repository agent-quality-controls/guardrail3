pub mod command_query;
mod support;

use self::support::*;

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

impl ExecutableLine {
    #[must_use]
    pub fn line_no(&self) -> usize {
        self.line_no
    }

    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub fn command_text(&self) -> &str {
        &self.command_text
    }

    #[must_use]
    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    #[must_use]
    pub fn softened_by(&self) -> Option<&FailOpenWrapper> {
        self.softened_by.as_ref()
    }

    #[must_use]
    pub fn is_dispatcher_syntax(&self) -> bool {
        self.is_dispatcher_syntax
    }

    #[must_use]
    pub fn is_exit_zero(&self) -> bool {
        self.is_exit_zero
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLine {
    pub line_no: usize,
    pub raw: String,
}

impl SourceLine {
    #[must_use]
    pub fn line_no(&self) -> usize {
        self.line_no
    }

    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedShellScript {
    pub shebang: Option<String>,
    pub source_lines: Vec<SourceLine>,
    pub executable_lines: Vec<ExecutableLine>,
    pub functions: Vec<ShellFunction>,
}

impl ParsedShellScript {
    #[must_use]
    pub fn shebang(&self) -> Option<&str> {
        self.shebang.as_deref()
    }

    #[must_use]
    pub fn source_lines(&self) -> &[SourceLine] {
        self.source_lines.as_slice()
    }

    #[must_use]
    pub fn executable_lines(&self) -> &[ExecutableLine] {
        self.executable_lines.as_slice()
    }

    #[must_use]
    pub fn functions(&self) -> &[ShellFunction] {
        self.functions.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFunction {
    pub name: String,
    pub line_no: usize,
    pub body: String,
    pub body_starts_on_definition_line: bool,
}

impl ShellFunction {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn line_no(&self) -> usize {
        self.line_no
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    #[must_use]
    pub fn body_starts_on_definition_line(&self) -> bool {
        self.body_starts_on_definition_line
    }
}

#[must_use]
pub fn parse_script(content: &str) -> ParsedShellScript {
    let mut shebang = None;
    let logical_lines = collect_logical_lines(content);
    let source_lines = content
        .lines()
        .enumerate()
        .map(|(index, raw)| SourceLine {
            line_no: index + 1,
            raw: raw.to_owned(),
        })
        .collect();
    let mut executable_lines = Vec::new();
    let mut functions = Vec::new();
    let mut function_brace_depth = 0usize;
    let mut current_function: Option<ShellFunction> = None;
    let mut dead_if_depth = 0usize;
    let mut dead_else_depth = 0usize;
    let mut dead_loop_depth = 0usize;
    let mut live_true_if_depth = 0usize;

    for (line_no, raw) in &logical_lines {
        let line_no = *line_no;
        let raw = raw.as_str();
        let trimmed = raw.trim();
        if line_no == 1 && trimmed.starts_with("#!") {
            shebang = Some(trimmed.to_owned());
            continue;
        }
        if let Some(current) = current_function.as_mut() {
            append_function_body_line(current, raw);
            function_brace_depth = update_function_scope_depth(function_brace_depth, trimmed);
            if function_brace_depth == 0
                && let Some(function) = current_function.take()
            {
                functions.push(function);
            }
            continue;
        }
        if let Some(taken_branch) = single_line_constant_if_taken_branch(raw) {
            if let Some(branch) = taken_branch {
                for executable in parse_executable_segments(branch, line_no) {
                    executable_lines.push(executable);
                }
            }
            continue;
        }
        if dead_else_depth > 0 {
            dead_else_depth = update_dead_else_depth(dead_else_depth, trimmed);
            continue;
        }
        if function_brace_depth > 0 {
            function_brace_depth = update_function_scope_depth(function_brace_depth, trimmed);
            continue;
        }
        if dead_if_depth > 0 {
            dead_if_depth = update_dead_if_depth(dead_if_depth, trimmed);
            continue;
        }
        if dead_loop_depth > 0 {
            dead_loop_depth = update_dead_loop_depth(dead_loop_depth, trimmed);
            continue;
        }
        if is_function_definition_line(trimmed) {
            function_brace_depth = function_scope_depth_after_definition(trimmed);
            let body = initial_function_body_fragment(raw);
            let function = ShellFunction {
                name: function_definition_name(trimmed).unwrap_or_default(),
                line_no,
                body_starts_on_definition_line: !body.is_empty(),
                body,
            };
            if function_brace_depth == 0 {
                functions.push(function);
                if let Some(tail) = inline_command_after_function_definition(raw)
                    && let Some(executable) = parse_executable_line(tail, line_no)
                {
                    executable_lines.push(executable);
                }
            } else {
                current_function = Some(function);
            }
            continue;
        }
        if starts_dead_if_scope(trimmed) {
            dead_if_depth = dead_if_scope_depth_after_start(trimmed);
            continue;
        }
        if live_true_if_depth > 0 && starts_dead_alternate_if_branch(trimmed) {
            live_true_if_depth = live_true_if_depth.saturating_sub(1);
            dead_else_depth = 1;
            continue;
        }
        if starts_live_true_if_scope(trimmed) {
            live_true_if_depth += 1;
        } else if live_true_if_depth > 0 && is_fi_line(trimmed) {
            live_true_if_depth = live_true_if_depth.saturating_sub(1);
        }
        if starts_dead_loop_scope(trimmed) {
            dead_loop_depth = dead_loop_scope_depth_after_start(trimmed);
            continue;
        }
        let Some(executable) = parse_executable_line(raw, line_no) else {
            continue;
        };
        executable_lines.push(executable);
    }

    ParsedShellScript {
        shebang,
        source_lines,
        executable_lines,
        functions,
    }
}

#[cfg(test)]
mod tests;
