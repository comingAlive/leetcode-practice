import {performance} from "perf_hooks";

function rob(nums: number[]): number {
    let dp: number[] = [0, 0]
    for (let i = 0; i < nums.length; i++) {
        [dp[0], dp[1]] = [dp[1], Math.max(dp[1], dp[0] + nums[i])]
    }
    return dp[1]
};

const n = [2, 7, 9, 3, 1];

const start = performance.now();
const result = rob(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
