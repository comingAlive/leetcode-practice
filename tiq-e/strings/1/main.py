from time import perf_counter
from typing import List


class Solution:
    def reverseString(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        half, max = len(s) >> 1, len(s) - 1
        for i in range(half):
            s[i], s[max - i] = s[max - i], s[i]


arr = ['h', 'e', 'l', 'l', 'o']

start = perf_counter()
Solution().reverseString(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(arr))
