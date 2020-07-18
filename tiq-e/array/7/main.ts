import {performance} from 'perf_hooks';

function plusOne(digits: number[]): number[] {
  for (let i = digits.length - 1; i >= 0; i--) {
    if (digits[i] < 9) {
      digits[i] += 1
      return digits
    }

    digits[i] = 0
  }
  digits.unshift(1)
  return digits
};

let arr = [3, 1, 2, 1, 2];

let start = performance.now();
let result = plusOne(arr);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
