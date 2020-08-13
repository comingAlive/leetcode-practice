use std::time::Instant;

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            k => (2..k).fold((1, 2), |acc, __| (acc.1, acc.0 + acc.1)).1,
        }
    }
}

fn main() {
    let n = 5;

    let start = Instant::now();
    let result = Solution::climb_stairs(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
