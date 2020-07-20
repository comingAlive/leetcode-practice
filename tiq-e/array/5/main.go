package main

import (
	"fmt"
	"time"
)

func singleNumber(nums []int) int {
	var out int
	for _, num := range nums {
		out ^= num
	}
	return out
}

func main() {
	arr := []int{3, 2, 2, 6, 3}

	var start = time.Now()
	var result = singleNumber(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
