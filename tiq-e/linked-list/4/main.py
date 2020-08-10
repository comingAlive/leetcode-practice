from collections import deque
from time import perf_counter
from typing import List


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        head = None
        if l1 is None:
            return l2
        if l2 is None:
            return l1   
        if l1.val < l2.val:
            head = l1
            l1 = l1.next
        else:
            head = l2
            l2 = l2.next

        head.next = self.mergeTwoLists(l1, l2)
        return head


first = ListNode(1, None)
second = ListNode(1, None)

start = perf_counter()
result = Solution().mergeTwoLists(first, second)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
