import { performance } from "perf_hooks";

function merge(nums1: number[], m: number, nums2: number[], n: number): void {
  let indexOfN = 0;
  nums1.forEach((part: number, index: number) => {
    if (index > m - 1) {
      nums1[index] = nums2[indexOfN];
      indexOfN = indexOfN + 1;
    }
  });
  nums1.sort((a: number, b: number) => a - b);
}

const nums1 = [1, 2, 3, 0, 0, 0];
const nums2 = [2, 5, 6];

const start = performance.now();
merge(nums1, 3, nums2, 3);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", nums1);
