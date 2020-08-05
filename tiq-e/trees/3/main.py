from time import perf_counter


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def isSymmetric(self, root: TreeNode) -> bool:
        def isMirror(tree1, tree2):
            if not tree1 and not tree2:
                return True
            if not tree1 or not tree2:
                return False
            if tree1.val != tree2.val:
                return False
            return isMirror(tree1.left, tree2.right) and isMirror(tree1.right, tree2.left)

        if not root:
            return True
        return isMirror(root.left, root.right)


root = TreeNode(5, None, None)

start = perf_counter()
result = Solution().isSymmetric(root)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
