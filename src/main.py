from time import perf_counter
from typing import List


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        if len(nums) < 2:
            return len(nums)

        u = 0
        for num in nums[1:]:
            if num != nums[u]:
                u += 1
                nums[u] = num

        return u + 1


nums = [1, 1, 2]

start = perf_counter()
result = Solution().removeDuplicates(nums)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
