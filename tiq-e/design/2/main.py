from time import perf_counter
from typing import List


class MinStack:
    def __init__(self):
        """
        initialize your data structure here.
        """
        self.stack = []
        self.min = []

    def push(self, x: int) -> None:
        self.stack.append(x)
        if not self.min or x <= self.min[-1]:
            self.min.append(x)

    def pop(self) -> None:
        if self.stack[-1] == self.min[-1]:
            self.min.pop(-1)
        self.stack.pop(-1)

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.min[-1]


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(x)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()

start = perf_counter()
result = MinStack()

result.push(1)
result.push(6)
result.push(3)
result.top()
result.getMin()
result.pop()

end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result.stack))
