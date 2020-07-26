import {performance} from 'perf_hooks';

function reverse(x: number): number {
    let sign = 1
    if (x < 0) {
        sign = -1
        x = x * -1
    }
    let res = 0;
    while (x) {
        const pop = x % 10;
        res = res * 10 + pop
        x = Math.trunc(x / 10);
    }
    if (res > 2 ** 31) {
        return 0;
    }
    return sign * res
}

let num = 123

let start = performance.now();
let result = reverse(num);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
