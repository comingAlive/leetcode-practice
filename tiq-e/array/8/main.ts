import {performance} from 'perf_hooks';

function moveZeroes(nums: number[]): void {
  let j = 0
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] !== 0) {
      [nums[j], nums[i]] = [nums[i], nums[j]]
      j += 1
    }
  }
}

let arr = [0, 1, 0, 3, 12];

let start = performance.now();
moveZeroes(arr);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', arr)
