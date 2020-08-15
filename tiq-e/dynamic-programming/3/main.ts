import {performance} from "perf_hooks";

function maxSubArray(nums: number[]): number {
    let current = nums[0]
    let result = nums[0]
    for (let i = 1; i < nums.length; i++) {
        current = Math.max(nums[i], current+nums[i])
        result = Math.max(result, current)
    }
    return result
}

const n = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

const start = performance.now();
const result = maxSubArray(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
