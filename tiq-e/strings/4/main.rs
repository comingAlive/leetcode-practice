use std::collections::HashMap;
use std::time::Instant;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();

        for letter in s.chars() {
            *s_map.entry(letter).or_insert(0) += 1;
        }
        for letter in t.chars() {
            *t_map.entry(letter).or_insert(0) += 1;
        }
        return s_map == t_map;
    }
}

fn main() {
    let s: String = "ab".to_string();
    let t: String = "a".to_string();

    let start = Instant::now();
    let result = Solution::is_anagram(s, t);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
