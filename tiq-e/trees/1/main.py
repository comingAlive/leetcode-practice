from time import perf_counter


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def maxDepth(self, root: TreeNode) -> int:
        if not root:
            return 0

        left_depth = self.maxDepth(root.left)
        right_depth = self.maxDepth(root.right)

        if left_depth > right_depth:
            return left_depth + 1
        else:
            return right_depth + 1


root = TreeNode(5, None, None)

start = perf_counter()
result = Solution().maxDepth(root)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
