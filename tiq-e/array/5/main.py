from time import perf_counter
from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        n = 0
        for num in nums:
            n = n ^ num
        return n


arr = [4, 1, 2, 1, 2]

start = perf_counter()
result = Solution().singleNumber(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
