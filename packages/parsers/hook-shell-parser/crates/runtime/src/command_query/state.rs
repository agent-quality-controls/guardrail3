#[derive(Debug, Clone)]
pub(super) struct TokenCursor<'a> {
    tokens: &'a [String],
    index: usize,
}

impl<'a> TokenCursor<'a> {
    pub(super) fn new(tokens: &'a [String]) -> Self {
        Self { tokens, index: 0 }
    }

    pub(super) fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.index).map(String::as_str)
    }

    pub(super) fn next(&mut self) -> Option<&'a str> {
        let token = self.peek()?;
        self.index += 1;
        Some(token)
    }

    pub(super) fn remaining(&self) -> &'a [String] {
        self.tokens.get(self.index..).unwrap_or(&[])
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct NoEnvState;

impl super::ShellEnvState for NoEnvState {
    fn apply_assignment(&mut self, _name: &str, _value: &str) {}

    fn unset(&mut self, _name: &str) {}

    fn clear(&mut self) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SegmentOutcome {
    pub(super) stopped: bool,
    pub(super) persist_state: bool,
}

pub(super) fn apply_assignment_token<S: super::ShellEnvState>(token: &str, state: &mut S) {
    let Some((name, value)) = token.split_once('=') else {
        return;
    };
    state.apply_assignment(name, value);
}

pub(super) fn apply_export_assignments<S: super::ShellEnvState>(
    cursor: &mut TokenCursor<'_>,
    state: &mut S,
) {
    while let Some(token) = cursor.next() {
        apply_assignment_token(token, state);
    }
}

pub(super) fn apply_unset_arguments<S: super::ShellEnvState>(
    cursor: &mut TokenCursor<'_>,
    state: &mut S,
) {
    while let Some(token) = cursor.next() {
        if token.starts_with('-') {
            continue;
        }
        state.unset(token);
    }
}
