import {performance} from 'perf_hooks';

function containsDuplicate(nums: number[]): boolean {
  let set = new Set(nums);
  return set.size !== nums.length;
}

function run(testFunction: (nums: number[]) => boolean, testArray: number[]) {
  let start = performance.now();
  let result = testFunction(testArray);
  let end = performance.now();
  console.log('Duration:', end - start);
  console.log('Result:', result)
}

let arr = [1, 2, 3, 4];
let setSize = 100000;

for (let i = 0; i < setSize; i++) arr.push(i);

run(containsDuplicate, arr);
