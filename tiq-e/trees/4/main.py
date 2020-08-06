from collections import deque
from time import perf_counter
from typing import List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def levelOrder(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []
        res = []
        queue = deque([root])
        while queue:
            row = []
            for _ in range(len(queue)):
                root = queue.popleft()
                row.append(root.val)
                if root.left:
                    queue.append(root.left)
                if root.right:
                    queue.append(root.right)
            res.append(row)
        return res


root = TreeNode(5, None, None)

start = perf_counter()
result = Solution().levelOrder(root)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
