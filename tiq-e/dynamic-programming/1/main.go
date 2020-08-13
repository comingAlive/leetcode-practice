package main

import (
	"fmt"
	"time"
)

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	a, b := 1, 2
	for i := 3; i < n; i++ {
		tmp := b
		b = b + a
		a = tmp
	}
	return a + b
}

func main() {
	var n = 5

	var start = time.Now()
	var result = climbStairs(n)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
