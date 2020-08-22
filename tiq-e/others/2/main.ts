import {performance} from "perf_hooks";

function hammingDistance(x: number, y: number): number {
    let ans = 0
    for (let i = 0; i < 32; i++) {
        let xBit = x & 1
        let yBit = y & 1

        ans += xBit !== yBit ? 1 : 0

        x >>= 1
        y >>= 1
    }
    return ans
}

const x = 2;
const y = 4;

const start = performance.now();
const result = hammingDistance(x, y);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
