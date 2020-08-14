use std::time::Instant;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut minprice = std::i32::MAX;
        let mut maxprofit = 0;
        for price in prices {
            if price < minprice {
                minprice = price
            } else if price - minprice > maxprofit {
                maxprofit = price - minprice
            }
        }
        maxprofit
    }
}

fn main() {
    let n = Vec::from([7, 1, 5, 3, 6, 4]);

    let start = Instant::now();
    let result = Solution::max_profit(n);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
