from time import perf_counter
from typing import List


class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        if not numRows:
            return []
        ans = [[1]]
        for i in range(1, numRows):
            li = [1]
            for j in range(len(ans[-1]) - 1):
                li.append(ans[-1][j] + ans[-1][j + 1])
            li.append(1)
            ans.append(li)
        return ans


x = 5

start = perf_counter()
result = Solution().generate(x)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
