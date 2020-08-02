import {performance} from "perf_hooks";

function maxProfit(prices: number[]): number {
    let profit = 0;

    for (let i = 1; i < prices.length; i++) {
        if (prices[i] > prices[i - 1]) {
            profit += prices[i] - prices[i - 1];
        }
    }

    return profit;
}

const prices = [7, 1, 5, 3, 6, 4];

const start = performance.now();
const result = maxProfit(prices);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
