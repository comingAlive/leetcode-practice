from time import perf_counter


class Solution:
    def countPrimes(self, n: int) -> int:
        if n < 2:
            return 0
        strikes = [1] * n

        strikes[0] = 0
        strikes[1] = 0

        for i in range(2, int(n ** 0.5) + 1):
            if strikes[i] != 0:
                strikes[i * i:n:i] = [0] * ((n - 1 - i * i) // i + 1)

        return sum(strikes)


n = 10

start = perf_counter()
result = Solution().countPrimes(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
