from collections import deque
from time import perf_counter
from typing import List


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def removeNthFromEnd(self, head, n):
        fast = slow = head
        print(fast, slow)
        for _ in range(n):
            fast = fast.next
        if not fast:
            return head.next
        while fast.next:
            fast = fast.next
            slow = slow.next
        slow.next = slow.next.next
        return head


head = ListNode(1, None)

start = perf_counter()
result = Solution().removeNthFromEnd(head, 1)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
