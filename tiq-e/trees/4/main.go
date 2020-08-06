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

func levelOrder(root *TreeNode) [][]int {
	var result [][]int
	if root == nil {
		return result
	}
	level := []*TreeNode{root}
	for len(level) > 0 {
		var tmp1 []*TreeNode
		var tmp2 []int
		for _, c := range level {
			if c != nil {
				tmp2 = append(tmp2, c.Val)
				tmp1 = append(tmp1, c.Left)
				tmp1 = append(tmp1, c.Right)
			}
		}
		if len(tmp2) != 0 {
			result = append(result, tmp2)
		}
		level = tmp1

	}
	return result
}

func main() {
	var root = TreeNode{
		Val:   5,
		Left:  nil,
		Right: nil,
	}

	var start = time.Now()
	var result = levelOrder(&root)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
