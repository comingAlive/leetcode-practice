use std::time::Instant;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for char in s.chars() {
            match char {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(char) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
    }
}

fn main() {
    let x: String = String::from("()[]{}");

    let start = Instant::now();
    let result = Solution::is_valid(x);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
