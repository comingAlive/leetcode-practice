from random import random
from time import perf_counter
from typing import List


class Solution:
    def fizzBuzz(self, n):
        return ['Fizz' * (not i % 3) + 'Buzz' * (not i % 5) or str(i) for i in range(1, n + 1)]


n = 15

start = perf_counter()
result = Solution().fizzBuzz(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
