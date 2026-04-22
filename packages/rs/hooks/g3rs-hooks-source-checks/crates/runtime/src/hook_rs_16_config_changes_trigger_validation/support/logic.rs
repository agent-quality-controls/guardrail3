use hook_shell_parser::types::ParsedShellScript;

use crate::hook_rs_08_guardrail_validate_staged_present::{
    line_contains_guardrail_step, line_contains_path_qualified_guardrail_step,
};
use super::text::{
    expand_inline_case_block, expand_inline_if_block, line_contains_then,
    line_reaches_config_trigger, looks_like_case_pattern_line,
};

const CONFIG_NEEDLES: [&str; 8] = [
    "guardrail3-rs.toml",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "rustfmt.toml",
    ".rustfmt.toml",
    "rust-toolchain.toml",
];

pub(crate) fn missing_config_needles(parsed: &ParsedShellScript) -> Vec<&'static str> {
    let blocks = conditional_blocks(parsed);

    CONFIG_NEEDLES
        .iter()
        .copied()
        .filter(|needle| {
            !(blocks.iter().any(|block| {
                block_branches(block)
                    .into_iter()
                    .any(|branch| branch_covers_needle(parsed, &branch, needle))
            }) || content_has_direct_trigger_line_for_needle(parsed, needle))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NumberedLine {
    line_no: usize,
    raw: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct NumberedBlock {
    lines: Vec<NumberedLine>,
}

fn conditional_blocks(parsed: &ParsedShellScript) -> Vec<NumberedBlock> {
    let mut blocks = Vec::new();
    let mut current = Vec::new();
    let mut depth = 0usize;

    for line in &parsed.source_lines {
        let trimmed = line.raw.trim();
        if depth == 0 && starts_conditional_block(trimmed) {
            current.push(NumberedLine {
                line_no: line.line_no,
                raw: line.raw.clone(),
            });
            if ends_conditional_block(trimmed) {
                blocks.push(NumberedBlock {
                    lines: std::mem::take(&mut current),
                });
                continue;
            }

            depth = 1;
            continue;
        }

        if depth > 0 {
            current.push(NumberedLine {
                line_no: line.line_no,
                raw: line.raw.clone(),
            });
            if starts_conditional_block(trimmed) {
                depth += 1;
            } else if ends_conditional_block(trimmed) {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    blocks.push(NumberedBlock {
                        lines: std::mem::take(&mut current),
                    });
                }
            }
        }
    }

    blocks
}

fn starts_conditional_block(line: &str) -> bool {
    line.starts_with("if ") || line.starts_with("case ")
}

fn ends_conditional_block(line: &str) -> bool {
    line == "fi"
        || line.ends_with("; fi")
        || line.ends_with(";fi")
        || line == "esac"
        || line.ends_with("; esac")
        || line.ends_with(";esac")
}

fn block_contains_validation(parsed: &ParsedShellScript, block: &NumberedBlock) -> bool {
    block.lines.iter().any(|line| {
        line_contains_guardrail_step(parsed, &line.raw, line.line_no)
            || line_contains_path_qualified_guardrail_step(parsed, &line.raw, line.line_no)
    })
}

fn branch_covers_needle(parsed: &ParsedShellScript, branch: &NumberedBlock, needle: &str) -> bool {
    let (direct_branch, nested_blocks) = partition_branch(branch);
    (block_contains_validation(parsed, &direct_branch)
        && block_mentions_config_trigger(parsed, &direct_branch, needle))
        || nested_blocks.into_iter().any(|nested_block| {
            block_branches(&nested_block)
                .into_iter()
                .any(|nested_branch| branch_covers_needle(parsed, &nested_branch, needle))
        })
}

fn block_branches(block: &NumberedBlock) -> Vec<NumberedBlock> {
    let first_non_empty = block
        .lines
        .iter()
        .find_map(|line| {
            let trimmed = line.raw.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .unwrap_or_default();

    if first_non_empty.starts_with("if ") {
        return if_branches(block);
    }

    if first_non_empty.starts_with("case ") {
        return case_branches(block);
    }

    vec![block.clone()]
}

fn partition_branch(branch: &NumberedBlock) -> (NumberedBlock, Vec<NumberedBlock>) {
    let first_non_empty = branch
        .lines
        .iter()
        .find_map(|line| {
            let trimmed = line.raw.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .unwrap_or_default();
    let mut direct_lines = Vec::new();
    let mut nested_blocks = Vec::new();
    let mut current_nested = Vec::new();
    let mut nested_depth = 0usize;
    let mut in_if_condition =
        first_non_empty.starts_with("if ") || first_non_empty.starts_with("elif ");
    let mut saw_first_non_empty = false;

    for line in &branch.lines {
        let trimmed = line.raw.trim();

        if !saw_first_non_empty && !trimmed.is_empty() {
            saw_first_non_empty = true;
            direct_lines.push(line.clone());
            if in_if_condition && line_contains_then(trimmed) {
                in_if_condition = false;
            }
            continue;
        }

        if in_if_condition {
            direct_lines.push(line.clone());
            if line_contains_then(trimmed) {
                in_if_condition = false;
            }
            continue;
        }

        if nested_depth == 0 && starts_conditional_block(trimmed) {
            current_nested.push(line.clone());
            if ends_conditional_block(trimmed) {
                nested_blocks.push(NumberedBlock {
                    lines: std::mem::take(&mut current_nested),
                });
            } else {
                nested_depth = 1;
            }
            continue;
        }

        if nested_depth > 0 {
            current_nested.push(line.clone());
            if starts_conditional_block(trimmed) && !ends_conditional_block(trimmed) {
                nested_depth += 1;
            } else if ends_conditional_block(trimmed) {
                nested_depth = nested_depth.saturating_sub(1);
                if nested_depth == 0 {
                    nested_blocks.push(NumberedBlock {
                        lines: std::mem::take(&mut current_nested),
                    });
                }
            }
            continue;
        }

        direct_lines.push(line.clone());
    }

    (NumberedBlock { lines: direct_lines }, nested_blocks)
}

fn if_branches(block: &NumberedBlock) -> Vec<NumberedBlock> {
    if block.lines.len() == 1 {
        let expanded = expand_inline_if_block(&block.lines[0].raw);
        if expanded != block.lines[0].raw {
            return if_branches(&expanded_single_line_block(block.lines[0].line_no, &expanded));
        }
    }

    let mut lines = block.lines.iter();
    let Some(first_line) = lines.next().cloned() else {
        return Vec::new();
    };

    let mut branches = Vec::new();
    let mut current = vec![first_line.clone()];
    let mut depth = if ends_conditional_block(first_line.raw.trim()) {
        0usize
    } else {
        1usize
    };

    if depth == 0 {
        return vec![NumberedBlock { lines: current }];
    }

    for line in lines {
        let trimmed = line.raw.trim();
        if depth == 1 && (trimmed.starts_with("elif ") || trimmed == "else") {
            branches.push(NumberedBlock {
                lines: std::mem::take(&mut current),
            });
        }

        current.push(line.clone());
        depth = adjust_depth(depth, trimmed);

        if depth == 0 {
            branches.push(NumberedBlock {
                lines: std::mem::take(&mut current),
            });
            break;
        }
    }

    if !current.is_empty() {
        branches.push(NumberedBlock { lines: current });
    }

    branches
}

fn case_branches(block: &NumberedBlock) -> Vec<NumberedBlock> {
    if block.lines.len() == 1 {
        let expanded = expand_inline_case_block(&block.lines[0].raw);
        if expanded != block.lines[0].raw {
            return case_branches(&expanded_single_line_block(block.lines[0].line_no, &expanded));
        }
    }

    let mut lines = block.lines.iter();
    let Some(first_line) = lines.next().cloned() else {
        return Vec::new();
    };

    let mut branches = Vec::new();
    let mut current = Vec::new();
    let mut depth = if ends_conditional_block(first_line.raw.trim()) {
        0usize
    } else {
        1usize
    };

    if depth == 0 {
        return vec![NumberedBlock {
            lines: vec![first_line],
        }];
    }

    for line in lines {
        let trimmed = line.raw.trim();

        if depth == 1 && looks_like_case_pattern_line(trimmed) && !current.is_empty() {
            branches.push(NumberedBlock {
                lines: std::mem::take(&mut current),
            });
        }

        if !current.is_empty() || looks_like_case_pattern_line(trimmed) {
            current.push(line.clone());
        }

        depth = adjust_depth(depth, trimmed);

        if depth == 1 && trimmed == ";;" {
            branches.push(NumberedBlock {
                lines: std::mem::take(&mut current),
            });
            continue;
        }

        if depth == 0 {
            if !current.is_empty() {
                branches.push(NumberedBlock {
                    lines: std::mem::take(&mut current),
                });
            }
            break;
        }
    }

    branches
}

fn adjust_depth(depth: usize, trimmed: &str) -> usize {
    let mut next = depth;
    if starts_conditional_block(trimmed) && !ends_conditional_block(trimmed) {
        next += 1;
    } else if ends_conditional_block(trimmed) && next > 0 {
        next -= 1;
    }
    next
}

fn block_mentions_config_trigger(
    parsed: &ParsedShellScript,
    block: &NumberedBlock,
    needle: &str,
) -> bool {
    block.lines.iter().any(|line| {
        line_reaches_config_trigger(parsed, &line.raw, line.line_no, needle)
    })
}

fn content_has_direct_trigger_line_for_needle(parsed: &ParsedShellScript, needle: &str) -> bool {
    parsed.executable_lines.iter().any(|line| {
        line_reaches_config_trigger(parsed, &line.raw, line.line_no, needle)
            && (line_contains_guardrail_step(parsed, &line.raw, line.line_no)
                || line_contains_path_qualified_guardrail_step(parsed, &line.raw, line.line_no))
    })
}

fn expanded_single_line_block(line_no: usize, expanded: &str) -> NumberedBlock {
    NumberedBlock {
        lines: expanded
            .lines()
            .map(|raw| NumberedLine {
                line_no,
                raw: raw.to_owned(),
            })
            .collect(),
    }
}
