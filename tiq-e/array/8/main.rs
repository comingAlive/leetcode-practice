use std::time::Instant;

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(j, i);
                j += 1;
            }
        }
    }
}

fn main() {
    let mut arr = Vec::from([0, 1, 0, 3, 12]);

    let start = Instant::now();
    let result = Solution::move_zeroes(&mut arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", arr);
}