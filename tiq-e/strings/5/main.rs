use std::time::Instant;

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        s.chars()
            .zip(s.chars().rev())
            .fold(true, |acc, (a, b)| acc & (a == b))
    }
}

fn main() {
    let s = "ab_a".to_string();

    let start = Instant::now();
    let result = Solution::is_palindrome(s);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
