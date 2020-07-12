function removeDuplicates(nums: number[]): number {
  let i = 0;
  nums.forEach(function (elem) {
        if (elem !== nums[i]) {
            nums[++i] = elem;
        }
    });
    return nums.length && i + 1;
}

console.log(removeDuplicates([1, 2, 3, 3,4,4]))