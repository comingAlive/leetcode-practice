from time import perf_counter
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        rob1, rob2 = 0, 0

        for n in nums:
            temp = max(n + rob1, rob2)
            rob1 = rob2
            rob2 = temp
        return rob2


n = [2, 7, 9, 3, 1]

start = perf_counter()
result = Solution().rob(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
