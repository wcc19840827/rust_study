use sha256::{digest};

fn main() {
    // println!("Hello, world!");

    let base_str = "study blockchain!";

    let mut k = 0;
    loop {
        let s = format!("{}{}", base_str, k);

        let hash_val = digest(s);
        if hash_val.starts_with("0000") {
            println!("{}{} => {}", base_str, k, hash_val);
            break;
        }

        k += 1;
    }
}
