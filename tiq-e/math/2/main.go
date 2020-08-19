package main

import (
	"fmt"
	"time"
)

func countPrimes(n int) int {
	var cnt int
	data := make([]bool, n)
	for i := 2; i < n; i++ {
		if data[i] {
			continue
		}
		cnt++
		for j := i * i; j < n; j += i {
			data[j] = true
		}
	}
	return cnt
}

func main() {
	n := 15

	start := time.Now()
	result := countPrimes(n)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
