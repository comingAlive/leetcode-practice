import math
from time import perf_counter


class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        if n < 1:
            return False
        return round(math.log(n, 3), 9) == round(math.log(n, 3))


n = 27

start = perf_counter()
result = Solution().isPowerOfThree(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
