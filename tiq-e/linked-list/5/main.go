package main

import (
	"fmt"
	"time"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	head, slow, fast := nil, head, head
	for fast != nil && fast.Next != nil {
		head, slow.Next, slow, fast = slow, head, slow.Next, fast.Next.Next
	}
	if fast != nil {
		slow = slow.Next
	}
	for ; head != nil && head.Val == slow.Val; head, slow = head.Next, slow.Next {
	}
	return head == nil
}

func main() {
	var head = ListNode{Val: 1, Next: nil}

	var start = time.Now()
	var result = isPalindrome(&head)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
