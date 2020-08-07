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

func sortedArrayToBST(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}

	if len(nums) == 1 {
		return &TreeNode{nums[0], nil, nil}
	}

	mid := len(nums) / 2

	root := TreeNode{nums[mid], nil, nil}

	root.Left = sortedArrayToBST(nums[:mid])
	root.Right = sortedArrayToBST(nums[mid+1:])

	return &root
}

func main() {
	var root = []int{-10, -3, 0, 5, 9}
	var start = time.Now()
	var result = sortedArrayToBST(root)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
