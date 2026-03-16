// Attributes applied to expressions inside blocks — unusual scope.
// grep sees #[allow(unused)] and flags it. But the scope is extremely
// narrow: it only applies to the single let binding inside the block.

fn expression_level_allow() -> i32 {
    let x = {
        #[allow(unused_variables)] // reason: expression-level attribute test
        let y = 1;
        42
    };
    x
}

// Attribute on a match arm
fn match_arm_allow(val: Option<i32>) -> i32 {
    match val {
        #[allow(unused_variables)] // reason: match arm attribute test
        Some(inner) => 42,
        None => 0,
    }
}

// Attribute on a loop body expression
fn loop_allow() -> i32 {
    let mut sum = 0;
    for i in 0..10 {
        #[allow(clippy::identity_op)] // reason: loop body attribute test
        let val = i * 1;
        sum += val;
    }
    sum
}

fn main() {
    let _a = expression_level_allow();
    let _b = match_arm_allow(Some(5));
    let _c = loop_allow();
}
