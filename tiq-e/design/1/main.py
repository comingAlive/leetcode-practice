from random import random
from time import perf_counter
from typing import List


class Solution:

    def __init__(self, nums: List[int]):
        self.nums = nums

    def reset(self) -> List[int]:
        return self.nums
        """
        Resets the array to its original configuration and return it.
        """

    def shuffle(self) -> List[int]:
        return sorted(self.nums, key=lambda x: random())
        """
        Returns a random shuffling of the array.
        """


# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.reset()
# param_2 = obj.shuffle()

nums = [2, 7, 9, 3, 1]

obj = Solution(nums)
param_2 = obj.shuffle()

start = perf_counter()
result = param_2
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
