import {performance} from "perf_hooks";

function maxProfit(prices: number[]): number {
    let minprice = Infinity
    let maxprofit = 0
    for (let i = 0; i < prices.length; i++) {
        if (prices[i] < minprice) minprice = prices[i]
        else if (prices[i] - minprice > maxprofit) maxprofit = prices[i] - minprice
    }

    return maxprofit

}

const n = [7, 1, 5, 3, 6, 4];

const start = performance.now();
const result = maxProfit(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
