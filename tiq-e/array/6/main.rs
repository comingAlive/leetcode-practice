use std::time::Instant;

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut arr1: Vec<_> = nums1.into_iter().collect();

        for e1 in nums2.into_iter() {
            if let Some(pos) = arr1.iter().position(|e2| e1 == *e2) {
                result.push(e1);
                arr1.remove(pos);
            }
        }

        result
    }
}

fn main() {
    let arr = Vec::from([2, 3, 3, 2, 2]);
    let arr2 = Vec::from([1, 3, 3, 2, 2]);

    let start = Instant::now();
    let result = Solution::intersect(arr, arr2);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}