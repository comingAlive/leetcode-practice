use std::time::Instant;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1]
            }
        }

        return profit;
    }
}

fn main() {
    let prices = Vec::from([7, 1, 5, 3, 6, 4]);

    let start = Instant::now();
    let result = Solution::max_profit(prices);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
