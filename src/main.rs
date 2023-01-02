use sha256::{digest};

fn main() {
    // println!("Hello, world!");

    let base_str = "study blockchain!";

    for k in 0..10 {
        let s = format!("{}{}", base_str, k);

        let val = digest(s);
        println!("{}{} => {}", base_str, k, val);
    }
}
