import {performance} from "perf_hooks";

function fizzBuzz(n: number): string[] {
    return Array.from({length: n}, (v, k) => {
        const value = k + 1
        if (value % 15 === 0) {
            return "FizzBuzz"
        } else if (value % 3 === 0) {
            return "Fizz"
        } else if (value % 5 === 0) {
            return "Buzz"
        } else {
            return value.toString()
        }
    })
};

const n = 15;

const start = performance.now();
const result = fizzBuzz(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
