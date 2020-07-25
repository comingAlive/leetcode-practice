use std::time::Instant;

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (half, max) = (s.len() >> 1, s.len() - 1);
        for i in 0..half {
            s.swap(i, max - i)
        }
    }
}

fn main() {
    let mut arr = vec!['h', 'e', 'l', 'l', 'o'];

    let start = Instant::now();
    Solution::reverse_string(&mut arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", arr);
}
