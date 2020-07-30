package main

import (
	"fmt"
	"time"
)

func twoSum(nums []int, target int) []int {

	lookup := make(map[int]int)

	for i, v := range nums {
		j, ok := lookup[v]

		lookup[target-v] = i

		if ok {
			return []int{j, i}
		}

	}
	return []int{}

}

func main() {
	arr := []int{0, 1, 0, 3, 12}

	var start = time.Now()
	twoSum(arr, 9)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", arr)
}
