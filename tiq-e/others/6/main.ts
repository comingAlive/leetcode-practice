import {performance} from "perf_hooks";

function missingNumber(nums: number[]): number {
    let n: number = nums.length;
    let result: number = (n * (n + 1)) / 2;
    for (let i = 0; i < n; ++i) {
        result -= nums[i];
    }
    return result;
}

const x = [3, 0, 1];

const start = performance.now();
const result = missingNumber(x);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
