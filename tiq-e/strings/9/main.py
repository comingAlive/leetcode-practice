from time import perf_counter
from typing import List


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ""

        shortest = min(strs, key=len)
        print(shortest)
        for i, ch in enumerate(shortest):
            for other in strs:
                if other[i] != ch:
                    return shortest[:i]
        return shortest


arr = ["flower", "flow", "flight"]

start = perf_counter()
result = Solution().longestCommonPrefix(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
