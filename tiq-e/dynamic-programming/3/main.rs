use std::time::Instant;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut result = current;
        for &i in nums[1..].iter() {
            if current < 0 {
                current = i
            } else {
                current += i
            };
            result = std::cmp::max(result, current);
        }
        result
    }
}

fn main() {
    let n = Vec::from([-2, 1, -3, 4, -1, 2, 1, -5, 4]);

    let start = Instant::now();
    let result = Solution::max_sub_array(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
