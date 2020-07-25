import {performance} from 'perf_hooks';

/**
 Do not return anything, modify s in-place instead.
 */
function reverseString(s: string[]): void {
  const [half, max] = [s.length >> 1, s.length - 1]
  for (let i = 0; i < half; i++) {
    [s[i], s[max - i]] = [s[max - i], s[i]]
  }
}

let arr = ['h', 'e', 'l', 'l', 'o']

let start = performance.now();
reverseString(arr);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', arr)
