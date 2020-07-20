package main

import (
	"fmt"
	"time"
)

func plusOne(digits []int) []int {
	carry := 1

	for i := len(digits) - 1; i >= 0; i-- {
		digits[i], carry = (digits[i]+carry)%10, (digits[i]+carry)/10
		println(carry)
	}

	if carry != 0 {
		return append([]int{carry}, digits...)
	}

	return digits
}

func main() {
	arr := []int{3, 2, 2, 6, 4}

	var start = time.Now()
	var result = plusOne(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
