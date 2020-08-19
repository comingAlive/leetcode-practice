import {performance} from "perf_hooks";

function countPrimes(n: number): number {
    let flagArray = [],
        result = 0;
    for (let i = 2; i < n; i++) {
        if (flagArray[i] === undefined) {
            flagArray[i] = 1;
            result++;
            let j = 2;
            while (i * j < n) {
                flagArray[i * j] = 0;
                j++;
            }
        }
    }
    return result;
}

const n = 15;

const start = performance.now();
const result = countPrimes(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
