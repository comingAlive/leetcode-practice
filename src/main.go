package main

import (
	"fmt"
	"time"
)

func removeDuplicates(nums []int) int {
	if len(nums) < 2 {
		return len(nums)
	}

	u := 1
	for i := 1; i < len(nums); i++ {
		if nums[i-1] != nums[i] {
			nums[u] = nums[i]
			u++
		}
	}

	return u
}

func main() {
	var nums = []int{1, 1, 2}

	var start = time.Now()
	var result = removeDuplicates(nums)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
