import {performance} from 'perf_hooks';

function intersect(nums1: number[], nums2: number[]): number[] {
  const map = new Map();
  for (const n of nums1) {
    if (map.has(n)) {
      map.set(n, map.get(n) + 1);
    } else {
      map.set(n, 1);
    }
  }
  const result = [];
  for (const n of nums2)
    if (map.has(n) && map.get(n) > 0) {
      result.push(n);
      map.set(n, map.get(n) - 1);
    }
  return result;
}

let arr = [3, 1, 2, 1, 2];
let arr2 = [4, 1, 2, 1, 2];

let start = performance.now();
let result = intersect(arr, arr2);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
