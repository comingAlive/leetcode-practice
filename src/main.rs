use std::time::Instant;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut u = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[u] {
                u += 1;
                nums[u] = nums[i]
            }
        }

        return (u + 1) as i32;
    }
}

fn main() {
    let mut nums: Vec<i32> = Vec::from([1, 1, 2]);

    let start = Instant::now();
    let result = Solution::remove_duplicates(&mut nums);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
