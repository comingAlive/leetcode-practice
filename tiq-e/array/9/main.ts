import {performance} from 'perf_hooks';

function twoSum(nums: number[], target: number): number[] {
  const m: Map<number, number> = new Map();

  for (let i = 0; i < nums.length; i++) {
    if (m.has(nums[i])) {
      return [m.get(nums[i])!, i];
    }
    m.set(target - nums[i], i);
  }

  return [];
}

let arr = [2, 7, 11, 15];

let start = performance.now();
let result = twoSum(arr, 9);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
