from time import perf_counter
from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        d = {}
        for i in range(len(nums)):
            if nums[i] in d:
                return [d[nums[i]], i]
            else:
                d[target - nums[i]] = i
        return []


arr = [2, 7, 11, 15]

start = perf_counter()
result = Solution().twoSum(arr, 9)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
