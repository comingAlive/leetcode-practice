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

func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return isMirror(root.Left, root.Right)
}

func isMirror(tree1 *TreeNode, tree2 *TreeNode) bool {
	if tree1 == nil && tree2 == nil {
		return true
	}
	if tree1 == nil || tree2 == nil {
		return false
	}
	if tree1.Val != tree2.Val {
		return false
	}

	return tree1.Val == tree2.Val && isMirror(tree1.Left, tree2.Right) && isMirror(tree1.Right, tree2.Left)
}

func main() {
	var root = TreeNode{
		Val:   5,
		Left:  nil,
		Right: nil,
	}

	var start = time.Now()
	var result = isSymmetric(root)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
