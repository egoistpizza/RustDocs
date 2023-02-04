fn factorial(x: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else {
        x * factorial(x - 1.0)
    }
}

fn main() {
    println!("factorial of 0 is {}", factorial(0.0));
    println!("factorial of 5 is {}", factorial(5.0));
}
