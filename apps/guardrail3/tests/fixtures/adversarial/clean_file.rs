fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

fn main() {
    let _result = add(1, 2);
}
