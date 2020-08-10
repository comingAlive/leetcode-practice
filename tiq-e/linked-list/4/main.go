package main

import (
	"fmt"
	"time"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	var head *ListNode

	if l1 == nil {
		return l2
	}
	if l2 == nil {
		return l1
	}

	if l1.Val < l2.Val {
		head = l1
		l1 = l1.Next
	} else {
		head = l2
		l2 = l2.Next
	}
	head.Next = mergeTwoLists(l1, l2)
	return head
}

func main() {
	var first = ListNode{Val: 1, Next: nil}
	var second = ListNode{Val: 1, Next: nil}

	var start = time.Now()
	var result = mergeTwoLists(&first, &second)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
