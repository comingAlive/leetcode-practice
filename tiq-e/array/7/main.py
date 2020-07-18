from time import perf_counter
from typing import List


class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        for i in range(len(digits) - 1, -1, -1):
            if digits[i] != 9:
                digits[i] += 1
                break
            else:
                digits[i] = 0
                if i == 0:
                    digits.insert(0, 1)
        return digits


arr = [4, 1, 2, 1, 2]

start = perf_counter()
result = Solution().plusOne(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
