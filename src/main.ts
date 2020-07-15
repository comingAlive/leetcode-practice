import {performance} from 'perf_hooks';

function containsDuplicate(nums: number[]): boolean {
  let set = new Set(nums);
  return set.size !== nums.length;
}

let arr = [1, 2, 3, 3];

let start = performance.now();
let result = containsDuplicate(arr);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
