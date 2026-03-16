// This file contains ONLY comments — zero code lines.
// guardrail3 should report 0 effective lines.
// It should NOT fire R38 (file length > 500) even though
// you could add 600 comment lines here.

/* Block comment too */

/// Doc comment
/// with multiple lines
/// but still no code

// #[allow(dead_code)]
// The above looks like an allow attribute, but it's in a comment.
// grep might flag it. syn/AST should NOT.

// unsafe { std::ptr::null() }
// The above looks like unsafe code in a comment.

// use std::fs;
// The above looks like a filesystem import in a comment.

// todo!()
// The above looks like a todo macro in a comment.

// .unwrap()
// The above looks like an unwrap call in a comment.
