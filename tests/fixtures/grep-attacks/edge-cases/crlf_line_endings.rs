// This file uses Windows-style CRLF line endings

#[allow(dead_code)] // reason: testing CRLF handling
fn crlf_function() {
    let x = 42;
    println\!("{}", x);
}

#[allow(unused_variables)] // reason: CRLF test
fn another_function() {
    let y = "hello";
}

fn main() {
    crlf_function();
    another_function();
}
