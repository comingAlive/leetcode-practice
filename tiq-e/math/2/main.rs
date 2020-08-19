use std::time::Instant;

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut primes = n - 2;
        let mut is_prime = vec![true; n as usize];
        let mut i = 2;
        while i * i < n as usize {
            if is_prime[i] {
                let mut j = i * i;
                while j < n as usize {
                    if is_prime[j] {
                        is_prime[j] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }
        primes
    }
}

fn main() {
    let n = 10;

    let start = Instant::now();
    let result = Solution::count_primes(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
