#![allow(
    clippy::missing_docs_in_private_items,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::string_slice,
    reason = "shell_ast.rs IS the tree-sitter scaffolding; offsets, slicing, and arithmetic mirror tree-sitter's byte-position API"
)]

use tree_sitter::{Node, Parser, Tree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ShellCommandSegment {
    pub(super) text: String,
    pub(super) operator_before: Option<&'static str>,
    pub(super) operator_after: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HeredocTerminator {
    pub(super) value: String,
    pub(super) strip_tabs: bool,
}

pub(super) fn parse_tree(input: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .ok()?;
    parser.parse(input, None)
}

pub(super) fn strip_inline_comment(line: &str) -> &str {
    let Some(tree) = parse_tree(line) else {
        return line;
    };
    let Some(comment) = first_node_kind(tree.root_node(), "comment") else {
        return line;
    };
    line.get(..comment.start_byte()).unwrap_or(line)
}

pub(super) fn command_segments(input: &str) -> Vec<ShellCommandSegment> {
    let Some(tree) = parse_tree(input) else {
        return Vec::new();
    };

    let mut nodes = Vec::new();
    collect_executable_nodes(tree.root_node(), &mut nodes);
    nodes.sort_by_key(Node::start_byte);
    nodes.dedup_by_key(|node| (node.start_byte(), node.end_byte()));

    nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            let start_byte = leading_negation_start(input, node.start_byte());
            let text = input
                .get(start_byte..node.end_byte())
                .or_else(|| node_text(input, *node))?
                .trim()
                .to_owned();
            let operator_before = index
                .checked_sub(1)
                .and_then(|previous_index| nodes.get(previous_index))
                .and_then(|previous| separator_between(input, *previous, *node));
            let operator_after = nodes
                .get(index + 1)
                .and_then(|next| separator_between(input, *node, *next))
                .or_else(|| trailing_operator_after(input, *node));

            Some(ShellCommandSegment {
                text,
                operator_before,
                operator_after,
            })
        })
        .collect()
}

pub(super) fn shell_words(command_text: &str) -> Vec<String> {
    let Some(tree) = parse_tree(command_text) else {
        return Vec::new();
    };
    let root = tree.root_node();
    let Some(node) = first_shell_words_node(root) else {
        return Vec::new();
    };

    match node.kind() {
        "command" => command_words(command_text, node),
        "declaration_command" | "unset_command" | "test_command" => {
            keyword_command_words(command_text, node)
        }
        "variable_assignment" => node_text(command_text, node)
            .map(normalize_shell_word)
            .into_iter()
            .collect(),
        _ => Vec::new(),
    }
}

pub(super) fn command_substitutions(input: &str) -> Vec<String> {
    let Some(tree) = parse_tree(input) else {
        return Vec::new();
    };
    let mut nodes = Vec::new();
    collect_nodes_kind(tree.root_node(), "command_substitution", &mut nodes);
    nodes
        .into_iter()
        .filter_map(|node| command_substitution_text(input, node))
        .filter(|text| !text.is_empty())
        .collect()
}

pub(super) fn constant_exit_status(segment: &str) -> Option<bool> {
    let negated = negation_count(segment) % 2 == 1;
    let words = shell_words(segment);
    let first = words.first()?;
    let status = match first.as_str() {
        "true" | ":" => Some(true),
        "false" => Some(false),
        "exit" => Some(words.get(1).is_some_and(|arg| arg == "0")),
        _ => None,
    }?;
    Some(if negated { !status } else { status })
}

pub(super) fn is_terminal_exit(segment: &str) -> bool {
    shell_words(segment)
        .first()
        .is_some_and(|word| word.as_str() == "exit")
}

pub(super) fn leading_command_name(command_text: &str) -> Option<String> {
    shell_words(command_text).into_iter().next()
}

pub(super) fn command_substitution_assignment(input: &str) -> Option<String> {
    let tree = parse_tree(input)?;
    let root = tree.root_node();
    if !contains_assignment_node(root) {
        return None;
    }
    command_substitutions(input).into_iter().next()
}

pub(super) fn is_standalone_assignment(input: &str) -> bool {
    let Some(tree) = parse_tree(input) else {
        return false;
    };
    let root = tree.root_node();
    root.named_child_count() == 1
        && root.named_child(0).is_some_and(|node| {
            matches!(node.kind(), "variable_assignment" | "variable_assignments")
        })
}

pub(super) fn heredoc_terminator(line: &str) -> Option<HeredocTerminator> {
    let initial = parse_tree(line)?;
    let (tree, input) = if first_node_kind(initial.root_node(), "heredoc_redirect").is_some() {
        (initial, line.to_owned())
    } else {
        let synthetic = format!("{line}\n__G3_HEREDOC_BODY__\n__G3_HEREDOC_END__\n");
        (parse_tree(&synthetic)?, synthetic)
    };
    let redirect = first_node_kind(tree.root_node(), "heredoc_redirect")?;
    let start = first_node_kind(redirect, "heredoc_start")?;
    let marker = node_text(&input, start)?.trim();
    let strip_tabs = marker.starts_with("<<-");
    let delimiter = marker
        .trim_start_matches("<<-")
        .trim_start_matches("<<")
        .trim();
    let delimiter = strip_balanced_shell_quotes(delimiter);
    (!delimiter.is_empty()).then_some(HeredocTerminator {
        value: delimiter,
        strip_tabs,
    })
}

fn collect_executable_nodes<'tree>(node: Node<'tree>, out: &mut Vec<Node<'tree>>) {
    if matches!(
        node.kind(),
        "command" | "declaration_command" | "unset_command" | "test_command"
    ) {
        out.push(node);
        return;
    }
    if matches!(
        node.kind(),
        "command_substitution" | "process_substitution" | "function_definition" | "heredoc_body"
    ) {
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_named() {
            collect_executable_nodes(child, out);
        }
    }
}

fn leading_negation_start(input: &str, command_start: usize) -> usize {
    let prefix = input.get(..command_start).unwrap_or_default();
    let mut cursor = prefix.len();
    let mut saw_negation = false;

    loop {
        while cursor > 0 {
            let Some((previous_index, previous_char)) = prefix[..cursor].char_indices().next_back()
            else {
                break;
            };
            if !previous_char.is_whitespace() {
                break;
            }
            cursor = previous_index;
        }

        let Some((previous_index, previous_char)) = prefix[..cursor].char_indices().next_back()
        else {
            break;
        };
        if previous_char != '!' {
            break;
        }

        saw_negation = true;
        cursor = previous_index;
    }

    if saw_negation { cursor } else { command_start }
}

fn command_words(input: &str, node: Node<'_>) -> Vec<String> {
    let mut words = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "!" {
            continue;
        }
        if child.kind().contains("redirect") {
            continue;
        }
        if !child.is_named() {
            continue;
        }
        if child.kind() == "subshell" {
            continue;
        }
        if let Some(text) = node_text(input, child) {
            words.push(normalize_shell_word(text.trim()));
        }
    }
    words
}

fn child_words(input: &str, node: Node<'_>) -> Vec<String> {
    let mut words = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind().contains("redirect") || !child.is_named() {
            continue;
        }
        if let Some(text) = node_text(input, child) {
            words.push(normalize_shell_word(text.trim()));
        }
    }
    if words.is_empty() {
        if let Some(text) = node_text(input, node) {
            words.extend(
                text.split_whitespace()
                    .map(normalize_shell_word)
                    .filter(|word| !word.is_empty()),
            );
        }
    }
    words
}

fn keyword_command_words(input: &str, node: Node<'_>) -> Vec<String> {
    let mut words = Vec::new();
    if let Some(first_named) = node.named_child(0) {
        if let Some(prefix) = input.get(node.start_byte()..first_named.start_byte()) {
            if let Some(keyword) = prefix.split_whitespace().next() {
                words.push(normalize_shell_word(keyword));
            }
        }
    }
    words.extend(child_words(input, node));
    if words.is_empty() {
        if let Some(text) = node_text(input, node) {
            words.extend(
                text.split_whitespace()
                    .map(normalize_shell_word)
                    .filter(|word| !word.is_empty()),
            );
        }
    }
    words
}

fn first_shell_words_node(node: Node<'_>) -> Option<Node<'_>> {
    if matches!(
        node.kind(),
        "command"
            | "declaration_command"
            | "unset_command"
            | "test_command"
            | "variable_assignment"
    ) {
        return Some(node);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }
        if let Some(found) = first_shell_words_node(child) {
            return Some(found);
        }
    }
    None
}

fn command_substitution_text(input: &str, node: Node<'_>) -> Option<String> {
    let text = node_text(input, node)?.trim();
    if let Some(stripped) = text
        .strip_prefix("$(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        return Some(stripped.trim().to_owned());
    }
    if let Some(stripped) = text
        .strip_prefix('`')
        .and_then(|rest| rest.strip_suffix('`'))
    {
        return Some(stripped.trim().to_owned());
    }
    Some(text.to_owned())
}

fn negation_count(input: &str) -> usize {
    let Some(tree) = parse_tree(input) else {
        return 0;
    };
    count_nodes_kind(tree.root_node(), "negated_command")
}

fn contains_assignment_node(node: Node<'_>) -> bool {
    if matches!(node.kind(), "variable_assignment" | "variable_assignments") {
        return true;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(Node::is_named)
        .any(contains_assignment_node)
}

fn separator_between(input: &str, previous: Node<'_>, current: Node<'_>) -> Option<&'static str> {
    let between = input.get(previous.end_byte()..current.start_byte())?;
    compact_separator(between)
}

fn trailing_operator_after(input: &str, node: Node<'_>) -> Option<&'static str> {
    let tail = input.get(node.end_byte()..)?;
    let compact = tail
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();
    if compact.starts_with('&') && !compact.starts_with("&&") {
        return Some("&");
    }
    if compact.starts_with('|') && !compact.starts_with("||") {
        return Some("|");
    }
    None
}

fn compact_separator(input: &str) -> Option<&'static str> {
    let compact = input
        .chars()
        .filter(|ch| !ch.is_whitespace() && *ch != '(' && *ch != ')' && *ch != '{' && *ch != '}')
        .collect::<String>();
    if compact.contains("&&") {
        Some("&&")
    } else if compact.contains("||") {
        Some("||")
    } else if compact.contains('|') {
        Some("|")
    } else if compact.contains('&') {
        Some("&")
    } else if compact.contains(';') {
        Some(";")
    } else {
        None
    }
}

fn first_node_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }
        if let Some(found) = first_node_kind(child, kind) {
            return Some(found);
        }
    }
    None
}

fn collect_nodes_kind<'tree>(node: Node<'tree>, kind: &str, out: &mut Vec<Node<'tree>>) {
    if node.kind() == kind {
        out.push(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_named() {
            collect_nodes_kind(child, kind, out);
        }
    }
}

fn count_nodes_kind(node: Node<'_>, kind: &str) -> usize {
    let mut count = usize::from(node.kind() == kind);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_named() {
            count += count_nodes_kind(child, kind);
        }
    }
    count
}

fn node_text<'input>(input: &'input str, node: Node<'_>) -> Option<&'input str> {
    node.utf8_text(input.as_bytes()).ok()
}

fn strip_balanced_shell_quotes(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let Some(last) = value.chars().last() else {
        return String::new();
    };
    let char_count = value.chars().count();
    if char_count >= 2 && matches!((first, last), ('\'', '\'') | ('"', '"') | ('`', '`')) {
        return value.chars().skip(1).take(char_count - 2).collect();
    }
    value.to_owned()
}

fn normalize_shell_word(value: &str) -> String {
    if let Some((name, raw_value)) = value.split_once('=') {
        return format!("{name}={}", strip_balanced_shell_quotes(raw_value));
    }
    strip_balanced_shell_quotes(value)
}
