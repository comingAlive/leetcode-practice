from collections import deque
from time import perf_counter
from typing import List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> TreeNode:
        if not nums:
            return None

        mid = (len(nums) - 1) // 2

        root = TreeNode(nums[mid])

        root.left = self.sortedArrayToBST(nums[:mid])
        root.right = self.sortedArrayToBST(nums[mid + 1:])

        return root


root = [-10, -3, 0, 5, 9]

start = perf_counter()
result = Solution().sortedArrayToBST(root)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
