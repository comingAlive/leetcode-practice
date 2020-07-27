use std::collections::HashMap;
use std::time::Instant;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m: HashMap<char, i32> = HashMap::new();
        for l in s.chars() {
            if m.contains_key(&l) {
                m.insert(l, m.get(&l).unwrap() + 1);
            } else {
                m.insert(l, 1);
            }
        }
        for (i, l) in s.chars().enumerate() {
            if *m.get(&l).unwrap() == 1 {
                return i as i32;
            }
        }
        return -1;
    }
}

fn main() {
    let s: String = "loveleetcode".to_string();

    let start = Instant::now();
    let result = Solution::first_uniq_char(s);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
