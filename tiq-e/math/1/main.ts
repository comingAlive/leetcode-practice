import {performance} from "perf_hooks";

function fizzBuzz(n: number): string[] {
    let result = [];
    for (let i = 1; i < n + 1; i++) {
        if (!(i % 3) && !(i % 5)) {
            result.push('FizzBuzz')
        } else if (!(i % 3)) {
            result.push('Fizz')
        } else if (!(i % 5)) {
            result.push('Buzz')
        } else {
            result.push(i.toString())
        }
    }
    return result
};

const n = 15;

const start = performance.now();
const result = fizzBuzz(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
