use std::time::Instant;

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let mut x = x.abs() as u64;
        let mut res = 0;
        while x != 0 {
            let pop = x % 10;
            res = res * 10 + pop;
            x /= 10;
        }
        if res > i32::max_value() as u64 {
            return 0;
        }
        sign * res as i32
    }
}

fn main() {
    let num = 123;

    let start = Instant::now();
    let result = Solution::reverse(num);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
