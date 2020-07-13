struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        (1..prices.len())
            .map(|i| std::cmp::max(0, prices[i] - prices[i - 1]))
            .sum()
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]))
}