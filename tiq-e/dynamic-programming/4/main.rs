use std::time::Instant;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut former_max = 0;
        let mut curr_max = 0;
        for &num in nums.iter() {
            let mut temp = curr_max;
            curr_max = i32::max(former_max + num, curr_max);
            former_max = temp;
        }
        curr_max
    }
}

fn main() {
    let n = Vec::from([2, 7, 9, 3, 1]);

    let start = Instant::now();
    let result = Solution::rob(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
