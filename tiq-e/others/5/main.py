from time import perf_counter
from typing import List


class Solution:
    def isValid(self, s: str) -> bool:
        stack, match = [], {'{': '}', '[': ']', '(': ')'}
        for char in s:
            if char in match:
                stack.append(char)
            elif not stack or char != match[stack.pop()]:
                return False
        return not stack


x = "()[]{}"

start = perf_counter()
result = Solution().isValid(x)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
