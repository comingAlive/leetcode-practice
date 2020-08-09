package main

import (
	"fmt"
	"time"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var tail *ListNode
	var helper func(*ListNode, *ListNode)
	helper = func(prev, curr *ListNode) {
		if curr == nil {
			tail = prev
			return
		}

		helper(curr, curr.Next)
		curr.Next = prev
	}
	helper(nil, head)
	return tail
}

func main() {
	var head = ListNode{Val: 1, Next: nil}

	var start = time.Now()
	var result = reverseList(&head)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
