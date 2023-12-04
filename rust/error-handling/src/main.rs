fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero error")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let x = divide(10, 0);
    match x {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
