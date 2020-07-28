import collections
from time import perf_counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        c1 = collections.Counter(s)
        c2 = collections.Counter(t)
        return c1 == c2


s = "ba"
t = "a"

start = perf_counter()
result = Solution().isAnagram(s, t)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
