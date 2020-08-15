from time import perf_counter
from typing import List


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        maxsum = None
        start = 0
        subsum = 0
        for i, num in enumerate(nums):
            subsum = subsum + num
            if maxsum is None or subsum > maxsum:
                maxsum = subsum
            if subsum < 0:
                subsum = 0
                start = i + 1
        return maxsum


n = [-2, 1, -3, 4, -1, 2, 1, -5, 4]

start = perf_counter()
result = Solution().maxSubArray(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
