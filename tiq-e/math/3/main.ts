import {performance} from "perf_hooks";

function isPowerOfThree(n: number): boolean {
    return (Math.log(n) / Math.log(3)) % 1 < 0.00000000000001;
}

const n = 27;

const start = performance.now();
const result = isPowerOfThree(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
