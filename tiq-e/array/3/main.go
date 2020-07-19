package main

import (
	"fmt"
	"time"
)

func rotate(nums []int, k int) {
	l := len(nums)
	k = k % l
	newArr := append(nums[l-k:l], nums[0:l-k]...)
	copy(nums, newArr)
}

func main() {
	arr := []int{1, 1, 2, 3, 4}

	var start = time.Now()
	rotate(arr, 3)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", arr)
}
