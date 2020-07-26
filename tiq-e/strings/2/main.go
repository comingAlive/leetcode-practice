package main

import (
	"fmt"
	"math"
	"time"
)

func reverse(x int) int {
	sign := 1
	if x < 0 {
		sign = -1
		x = x * -1
	}
	var res int
	for x != 0 {
		pop := x % 10
		res = res*10 + pop
		x /= 10
	}
	if res > math.MaxInt32 {
		return 0
	}
	return sign * res
}

func main() {
	num := 123

	var start = time.Now()
	var result = reverse(num)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
