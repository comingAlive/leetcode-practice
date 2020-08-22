package main

import (
	"fmt"
	"time"
)

func hammingDistance(x int, y int) int {
	nCount := 0
	for r := x ^ y; r != 0; r >>= 1 {
		fmt.Println(r & 1)
		if r&1 == 1 {
			nCount++
		}
	}
	return nCount
}

func main() {
	x := 2
	y := 4

	start := time.Now()
	result := hammingDistance(x, y)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
