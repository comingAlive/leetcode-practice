package main

import (
	"fmt"
	"time"
)

func missingNumber(nums []int) int {
	var expected int
	var actual int
	for i := 0; i < len(nums); i++ {
		actual += nums[i]
		expected += i + 1
	}

	return expected - actual
}

func main() {
	x := []int{3, 0, 1}

	start := time.Now()
	result := missingNumber(x)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
