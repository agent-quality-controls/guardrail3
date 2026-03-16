// This file starts with a UTF-8 BOM

#[allow(dead_code)] // reason: testing BOM handling
fn bom_function() {
    let x = 42;
    println!("{}", x);
}

fn main() {
    bom_function();
}
