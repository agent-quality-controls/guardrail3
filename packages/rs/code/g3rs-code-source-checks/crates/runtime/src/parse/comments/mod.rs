/// Rule implementation for `scan`.
mod scan;

pub(crate) use scan::{
    effective_non_comment_line_count, line_text, same_line_has_comment, same_line_reason,
};
