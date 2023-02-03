// fn main() {
//     for i in 0..5 {
//         println!("{}", i);
//     }
// }

// fn main() {
//     for i in 0..10 {
//         if i % 2 == 0 {
//             println!("{} is even!", i);
//         } else {
//             println!("{} is odd!", i);
//         }
//     }
// }

fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}
