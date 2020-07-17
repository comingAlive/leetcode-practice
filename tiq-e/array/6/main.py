from time import perf_counter
from typing import List


class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        res = []
        dictionary = {}
        for n in nums1:
            if n in dictionary:
                dictionary[n] += 1
            else:
                dictionary[n] = 1
        for n in nums2:
            if n in dictionary:
                res.append(n)
                dictionary[n] -= 1
            if n in dictionary and dictionary[n] == 0:
                del dictionary[n]
        return res


arr = [4, 1, 2, 1, 2]
arr2 = [3, 1, 2, 1, 2]

start = perf_counter()
result = Solution().intersect(arr, arr2)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
