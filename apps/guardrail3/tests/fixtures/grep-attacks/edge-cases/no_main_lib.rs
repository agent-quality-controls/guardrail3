// A library-style file with no main function.
// guardrail3 should handle this fine — not all Rust files have main().

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[allow(dead_code)] // reason: lib-style unused function test
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub mod inner {
    #[allow(unused)] // reason: inner module test
    pub fn divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }
}
