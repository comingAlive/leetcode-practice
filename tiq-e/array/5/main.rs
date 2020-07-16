use std::time::Instant;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        for num in nums {
            n = n ^ num
        }
        return n;
    }
}

fn main() {
    let arr = Vec::from([1, 3, 3, 2, 2]);

    let start = Instant::now();
    let result = Solution::single_number(arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}