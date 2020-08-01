import {performance} from "perf_hooks";

function removeDuplicates(nums: number[]): number {
    if (nums.length < 2) {
        return nums.length;
    }

    let u = 0;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] !== nums[u]) {
            u += 1;
            [nums[u], nums[i]] = [nums[i], nums[u]];
        }
    }

    return u + 1;
}

const nums = [1, 1, 2];

const start = performance.now();
const result = removeDuplicates(nums);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
