from time import perf_counter


class Solution:
    def climbStairs(self, n: int) -> int:
        a, b = 1, 1
        for i in range(n):
            tmp = b
            b = a + b
            a = tmp
        return a


n = 5

start = perf_counter()
result = Solution().climbStairs(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
