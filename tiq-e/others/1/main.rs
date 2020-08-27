use std::time::Instant;

struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

fn main() {
    let n: u32 = 3;

    let start = Instant::now();
    let result = Solution::hammingWeight(n);

    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
