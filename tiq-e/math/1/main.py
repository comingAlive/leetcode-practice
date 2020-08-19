from time import perf_counter
from typing import List


class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        nums = []
        for num in range(1, n + 1):
            if num % 3 == 0 and num % 5 == 0:
                nums.append("FizzBuzz")
            elif num % 3 == 0:
                nums.append("Fizz")
            elif num % 5 == 0:
                nums.append("Buzz")
            else:
                nums.append(str(num))
        return nums


n = 15

start = perf_counter()
result = Solution().fizzBuzz(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
