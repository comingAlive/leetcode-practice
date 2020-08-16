package main

import (
	"fmt"
	"time"
)

func rob(nums []int) int {
	var prev1, prev2 int
	for _, num := range nums {
		prev1, prev2 = max(prev2+num, prev1), prev1
	}

	return prev1
}

func max(n1, n2 int) int {
	if n1 > n2 {
		return n1
	}
	return n2
}

func main() {
	var n = []int{2, 7, 9, 3, 1}

	var start = time.Now()
	var result = rob(n)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
