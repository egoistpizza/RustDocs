fn absVal(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn main() {
    let num1 = 5.0;
    let num2 = -5.0;
    if absVal(num1) == absVal(num2) {
        println!("|{}| = |{}|", num1, num2);
    }
}
