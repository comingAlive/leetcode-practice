use std::time::Instant;

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                return digits;
            }
        }
        digits.insert(0, 1);

        digits
    }
}

fn main() {
    let arr = Vec::from([2, 3, 3, 2, 9]);

    let start = Instant::now();
    let result = Solution::plus_one(arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}