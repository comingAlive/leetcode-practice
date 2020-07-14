from time import perf_counter
from typing import List

t0 = perf_counter()


class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        while k:
            nums.insert(0, nums.pop())
            k -= 1


n = [1, 2, 3, 4, 5, 6, 7]
Solution().rotate(n, 3)

t1 = perf_counter()
print(t1 - t0)

print(n)



