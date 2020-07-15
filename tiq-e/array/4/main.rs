use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = HashSet::from_iter(nums.clone());
        if set.len() != nums.len() {
            return true;
        }
        false
    }
}

fn main() {
    let arr = Vec::from([1, 2, 3, 3]);

    let start = Instant::now();
    let result = Solution::contains_duplicate(arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}