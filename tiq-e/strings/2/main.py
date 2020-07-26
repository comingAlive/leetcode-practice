from time import perf_counter


class Solution:
    def reverse(self, x: int) -> int:
        sign = 1
        if x < 0:
            sign = -1
            x = x * -1
        res = 0
        while x != 0:
            pop = x % 10
            res = res * 10 + pop
            x //= 10
        if res > 2 ** 31:
            return 0
        return sign * res


num = 123

start = perf_counter()
result = Solution().reverse(num)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
