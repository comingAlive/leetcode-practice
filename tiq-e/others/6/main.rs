use std::time::Instant;

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n + 1) / 2 - sum
    }
}

fn main() {
    let x = vec![3, 0, 1];

    let start = Instant::now();
    let result = Solution::missing_number(x);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
