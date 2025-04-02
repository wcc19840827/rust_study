fn main() {
    println!("Hello, world!");

    // let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut v1 = Vec::new();

    for k in 0..1000 {
        v1.push(k)
    }

    let v2: Vec<_> = v1.iter().map(|val|
    {
        println!("{}", val);
    
    }).collect();

    // assert_eq!(v2, vec![2, 3, 4]);
}


