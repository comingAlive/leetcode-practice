package main

import (
	"fmt"
	"time"
)

func maxSubArray(nums []int) int {
	current := 0
	result := nums[0]
	for i := range nums {
		current += nums[i]
		if current > result {
			result = current
		}
		if current < 0 {
			current = 0
		}
	}
	return result
}

func main() {
	var n = []int{-2, 1, -3, 4, -1, 2, 1, -5, 4}

	var start = time.Now()
	var result = maxSubArray(n)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
