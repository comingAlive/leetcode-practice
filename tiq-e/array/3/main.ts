function rotate(nums: number[], k: number): void {
  nums.unshift(...nums.splice(nums.length - k % nums.length));
}

let x = [1, 2, 3, 4, 5, 6, 7]
rotate(x, 3)
console.log(x)

const iterations = 1000000;
console.time('Function #1');
for (let i = 0; i < iterations; i++) {
  rotate(x, 3);
}
console.timeEnd('Function #1')

