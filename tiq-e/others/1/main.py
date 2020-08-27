from time import perf_counter


class Solution:
    def hammingWeight(self, n: int) -> int:
        num_of_1s = 0
        for i in range(64):
            num_of_1s += (n & 1)
            n = n >> 1
        return num_of_1s


n = 3

start = perf_counter()
result = Solution().hammingWeight(n)

end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
