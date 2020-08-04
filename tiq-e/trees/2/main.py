from time import perf_counter


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def isValidBST(self, root,  lessThan=float('inf'), largerThan=float('-inf')):
        if root is None:
            return True

        elif root.val >= lessThan or root.val <= largerThan:
            return False

        return self.isValidBST(root.left, min(lessThan, root.val), largerThan) and self.isValidBST(root.right, lessThan,
                                                                                                   max(largerThan,
                                                                                                       root.val))


root = TreeNode(5, None, None)

start = perf_counter()
result = Solution().isValidBST(root)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
