struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

fn main() {
    let z = Solution::remove_duplicates(&mut vec![1, 2, 3, 3, 3]);
    println!("{}", z)
}