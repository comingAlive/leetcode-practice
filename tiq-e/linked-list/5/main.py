from time import perf_counter


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        fast, slow = head, head
        while fast and fast.next:
            fast = fast.next.next
            slow = slow.next

        previous = None
        while slow:
            nextnode = slow.next
            slow.next = previous
            previous = slow
            slow = nextnode

        while previous:
            if previous.val != head.val:
                return False
            previous = previous.next
            head = head.next
        return True


head = ListNode(1, None)

start = perf_counter()
result = Solution().isPalindrome(head)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
