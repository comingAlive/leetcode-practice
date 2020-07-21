package main

import (
	"fmt"
	"time"
)

func moveZeroes(nums []int) {
	j := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] != 0 {
			nums[j] = nums[i]
			if j != i {
				nums[i] = 0
			}
			j += 1
		}
	}
}

func main() {
	arr := []int{0, 1, 0, 3, 12}

	var start = time.Now()
	moveZeroes(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", arr)
}
