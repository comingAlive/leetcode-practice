from time import perf_counter
from typing import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row = [set() for i in range(9)]
        col = [set() for i in range(9)]
        box = [set() for i in range(9)]

        for i in range(9):
            for j in range(9):
                cur = board[i][j]
                if cur != '.':

                    k = (i // 3) * 3 + j // 3

                    if cur not in row[i]:
                        row[i].add(cur)
                    else:
                        return False

                    if cur not in col[j]:
                        col[j].add(cur)
                    else:
                        return False

                    if cur not in box[k]:
                        box[k].add(cur)
                    else:
                        return False
        return True


arr = [
    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"]
]

start = perf_counter()
result = Solution().isValidSudoku(arr)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
