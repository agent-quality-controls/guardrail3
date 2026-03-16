// Adversarial fixture: R58 debatable case
// "use std::fs" is behind #[cfg(test)] — only compiled in test builds.
// Grep-based R58 will flag this unconditionally.
// Whether this SHOULD be flagged is debatable — test-only imports may be acceptable.

#[cfg(test)]
use std::fs;

fn main() {}

#[cfg(test)]
mod tests {
    fn read_fixture() -> String {
        super::fs::read_to_string("tests/fixture.txt").unwrap_or_default()
    }
}
