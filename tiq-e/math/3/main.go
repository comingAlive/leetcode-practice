package main

import (
	"fmt"
	"time"
)

func isPowerOfThree(n int) bool {
	a := 1
	for a < n {
		a *= 3
	}
	return a == n
}

func main() {
	n := 27

	start := time.Now()
	result := isPowerOfThree(n)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
