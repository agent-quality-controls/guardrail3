// Adversarial fixture: R58 false positive
// This is a re-export of crate::fs, NOT std::fs.
// Grep-based R58 may flag "use crate::fs" if it's too broad.
// AST-based check should NOT flag this — it's a crate-internal re-export.

mod fs {
    pub fn read_file() -> String {
        String::new()
    }
}

pub use crate::fs as filesystem;

fn main() {}
