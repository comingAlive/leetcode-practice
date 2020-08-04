package main

import (
	"fmt"
	"time"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	return RecValidate(root, nil, nil)
}

func RecValidate(n, min, max *TreeNode) bool {
	if n == nil {
		return true
	}
	if min != nil && n.Val <= min.Val {
		return false
	}
	if max != nil && n.Val >= max.Val {
		return false
	}
	return RecValidate(n.Left, min, n) && RecValidate(n.Right, n, max)
}

func main() {
	var root = TreeNode{
		Val:   5,
		Left:  nil,
		Right: nil,
	}

	var start = time.Now()
	var result = isValidBST(root)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
