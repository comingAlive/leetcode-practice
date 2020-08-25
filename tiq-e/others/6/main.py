from time import perf_counter
from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        result = (n * (n + 1)) // 2
        for num in nums:
            result -= num
        return result


x = [3, 0, 1]

start = perf_counter()
result = Solution().missingNumber(x)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
