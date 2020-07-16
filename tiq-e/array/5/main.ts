import {performance} from 'perf_hooks';

function singleNumber(nums: number[]): number {
  let n = 0
  for (let num of nums) {
    n = n ^ num
  }
  return n
}

let arr = [4, 1, 2, 1, 2];

let start = performance.now();
let result = singleNumber(arr);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
