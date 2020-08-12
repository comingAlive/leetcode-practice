from time import perf_counter
from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        for i in range(m, m + n):
            nums1[i] = nums2[i - m]
        nums1.sort()


nums1 = [1, 2, 3, 0, 0, 0]
nums2 = [2, 5, 6]

start = perf_counter()
Solution().merge(nums1, 3, nums2, 3)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(nums1))
