from time import perf_counter


class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        m = x ^ y
        b = bin(m)
        c = b[2::]
        d = 0
        for i in str(c):
            if i == '1':
                d += 1
        return d


x = 2
y = 4

start = perf_counter()
result = Solution().hammingDistance(x, y)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
