import {performance} from "perf_hooks";

function climbStairs(n: number): number {
  return Array(n - 1)
    .fill(0) 
    .reduce(([a, b]) => [b, a + b], [1, 1])
    .pop();
}

const n = 5;

const start = performance.now();
const result = climbStairs(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
