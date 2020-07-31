use std::time::Instant;

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let window_size = needle.len();
        let needle = needle.as_bytes();
        if window_size == 0 {
            return 0i32;
        }
        match haystack
            .as_bytes()
            .windows(window_size)
            .position(|slice| slice == needle)
        {
            Some(x) => x as i32,
            _ => -1i32,
        }
    }
}

fn main() {
    let haystack: String = "hello".to_string();
    let needle: String = "ll".to_string();

    let start = Instant::now();
    let result = Solution::str_str(haystack, needle);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
