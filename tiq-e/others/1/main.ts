import {performance} from "perf_hooks";

function hammingWeight(n: number): number {
    let sum = 0;
    while (n != 0) {
        sum += n & 1;
        n = n >>> 1;
    }
    return sum;
}

const n = 3;

const start = performance.now();
const result = hammingWeight(n);

const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
