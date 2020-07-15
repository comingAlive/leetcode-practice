from time import perf_counter
from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        s = set(nums)
        return len(s) != len(nums)


arr = [1, 2, 3, 4, 5]

start = perf_counter()
result = Solution().containsDuplicate(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
