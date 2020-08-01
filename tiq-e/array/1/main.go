package main

import (
	"fmt"
	"time"
)

func removeDuplicates(nums []int) int {
	if len(nums) < 2 {
		return len(nums)
	}

	u := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[u] {
			u += 1
			nums[u] = nums[i]
		}
	}

	return u + 1
}

func main() {
	var nums = []int{1, 1, 2}

	var start = time.Now()
	var result = removeDuplicates(nums)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
