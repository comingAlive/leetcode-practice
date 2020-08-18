import { performance } from "perf_hooks";

class Solution {
  arr: number[];
  origin: number[];

  constructor(nums: number[]) {
    this.arr = [...nums];
    this.origin = [...nums];
  }

  reset(): number[] {
    return this.origin;
  }

  shuffle(): number[] {
    const newArr: number[] = [];
    let ranNum: number;

    while (this.arr.length) {
      ranNum = Math.floor(Math.random() * this.arr.length);
      newArr.push(this.arr[ranNum]);
      this.arr.splice(ranNum, 1);
    }
    this.arr = [...this.origin];
    return newArr;
  }
}

/**
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(nums)
 * var param_1 = obj.reset()
 * var param_2 = obj.shuffle()
 */

const nums = [2, 7, 9, 3, 1];

const obj = new Solution(nums);
const param_2 = obj.shuffle();

const start = performance.now();
const result = param_2;
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
