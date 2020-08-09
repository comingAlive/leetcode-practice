from collections import deque
from time import perf_counter
from typing import List


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        t = None
        d = ListNode(-1)

        while head:
            th1 = th2 = head
            head = head.next

            th1.next = t
            t = th2

            if not head:
                d.next = th2
                break

        return d.next


head = ListNode(1, None)

start = perf_counter()
result = Solution().reverseList(head)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
