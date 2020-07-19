package main

import (
	"fmt"
	"time"
)

func containsDuplicate(nums []int) bool {
	m := make(map[int]bool)
	for _, i := range nums {
		if exist := m[i]; exist {
			return true
		}
		m[i] = true
	}
	return false
}

func main() {
	arr := []int{1, 5, 2, 3, 4}

	var start = time.Now()
	var result = containsDuplicate(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
