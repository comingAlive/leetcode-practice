use std::time::Instant;

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) -> Vec<i32> {
        nums1.truncate(m as usize);
        nums1.append(nums2);
        nums1.sort();
        nums1.to_vec()
    }
}

fn main() {
    let mut nums1 = Vec::from([1, 2, 3, 0, 0, 0]);
    let mut nums2 = Vec::from([2, 5, 6]);

    let start = Instant::now();
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", nums1);
}
