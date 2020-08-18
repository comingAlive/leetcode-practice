use std::time::Instant;

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let to_fizz_buzz = |x: i32| {
            if x % 15 == 0 {
                String::from("FizzBuzz")
            } else if x % 3 == 0 {
                String::from("Fizz")
            } else if x % 5 == 0 {
                String::from("Buzz")
            } else {
                x.to_string()
            }
        };

        (1..n + 1).map(to_fizz_buzz).collect()
    }
}

fn main() {
    let n = 15;

    let start = Instant::now();
    let result = Soluction::fizz_buzz(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
