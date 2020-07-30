use std::time::Instant;

struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let (n, s) = match str.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
            Some('+') => (1, 1),
            Some(x) if x.is_digit(10) => (0, 1),
            Some('-') => (1, -1),
            _ => return 0,
        };
        let mut res = 0i32;
        let overflow = if s > 0 { std::i32::MAX } else { std::i32::MIN };
        for c in str
            .chars()
            .skip_while(|x| x.is_whitespace())
            .skip(n)
            .take_while(|x| x.is_digit(10))
        {
            let (r, o) = res.overflowing_mul(10);
            if o {
                return overflow;
            }
            let (r, o) = r.overflowing_add(s * (c as i32 - '0' as i32));
            if o {
                return overflow;
            }
            res = r;
        }
        res
    }
}

fn main() {
    let s = "42".to_string();

    let start = Instant::now();
    let result = Solution::my_atoi(s);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
