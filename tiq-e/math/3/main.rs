use std::time::Instant;

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 3_i32.pow(19) % n == 0
    }
}

fn main() {
    let n = 27;

    let start = Instant::now();
    let result = Solution::count_primes(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
