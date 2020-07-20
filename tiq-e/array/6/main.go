package main

import (
	"fmt"
	"time"
)

func intersect(nums1 []int, nums2 []int) []int {
	cache := make(map[int]int)
	for _, v := range nums1 {
		cache[v]++
	}

	var res []int
	for _, v := range nums2 {
		if count, ok := cache[v]; ok {
			if count == 0 {
				continue
			}
			cache[v]--
			res = append(res, v)
		}
	}
	return res
}

func main() {
	arr := []int{3, 2, 2, 6, 4}
	arr2 := []int{3, 2, 2, 6, 3}

	var start = time.Now()
	var result = intersect(arr, arr2)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
