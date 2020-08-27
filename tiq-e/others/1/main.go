package main

import (
	"fmt"
	"time"
)

func hammingWeight(num uint32) int {
	res := 0
	for num > 0 {
		if num&1 == 1 {
			res++
		}
		num = num >> 1
	}
	return res
}

func main() {
	n := uint32(3)

	start := time.Now()
	result := hammingWeight(n)

	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
