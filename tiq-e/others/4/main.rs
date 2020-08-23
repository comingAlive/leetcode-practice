use std::time::Instant;

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for x in 0..num_rows as usize {
            res.push(vec![1; x + 1]);
            for y in 1..x {
                res[x][y] = res[x - 1][y - 1] + res[x - 1][y];
            }
        }
        return res;
    }
}

fn main() {
    let x = 5;

    let start = Instant::now();
    let result = Solution::generate(x);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
