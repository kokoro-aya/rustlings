pub fn factorial(num: u64) -> u64 {
    (1 .. num).reduce(|acc, x| acc * x  ).unwrap()
}
