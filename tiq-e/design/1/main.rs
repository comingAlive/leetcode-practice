use std::time::Instant;

use rand::seq::SliceRandom;

struct Solution {
    original: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { original: nums }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut array = self.original.clone();
        array.shuffle(&mut rng);
        array
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

fn main() {
    let nums = Vec::from([2, 7, 9, 3, 1]);

    let obj = Solution::new(nums);
    let ret_2: Vec<i32> = obj.shuffle();

    let start = Instant::now();
    let result = ret_2;
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
