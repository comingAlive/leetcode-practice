use std::time::Instant;

struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut r = 0;
        while z != 0 {
            r += 1;
            z = z & (z - 1);
        }
        r
    }
}

fn main() {
    let x = 2;
    let y = 4;

    let start = Instant::now();
    let result = Solution::hamming_distance(x, y);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
