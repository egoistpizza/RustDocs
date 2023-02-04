fn referance(x: &f64) -> f64 {
    *x + 3.0 as f64
}

fn main() {
    let x: f64 = 1.0;
    let reX = referance(&x);
    println!("x = {}, referance x = {}", x, reX);
}
