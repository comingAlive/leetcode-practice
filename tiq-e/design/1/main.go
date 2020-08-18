package main

import (
	"fmt"
	"math/rand"
	"time"
)

type Solution struct {
	original []int
}

func Constructor(nums []int) Solution {
	rand.Seed(time.Now().UnixNano())
	return Solution{
		original: nums,
	}
}

/** Resets the array to its original configuration and return it. */
func (s *Solution) Reset() []int {
	return s.original
}

/** Returns a random shuffling of the array. */
func (s *Solution) Shuffle() []int {
	shuffle := make([]int, len(s.original))
	copy(shuffle, s.original)
	for i := 0; i < len(shuffle)-1; i++ {
		index := rand.Intn(len(shuffle) - i)
		shuffle[len(shuffle)-1-i], shuffle[index] = shuffle[index], shuffle[len(shuffle)-1-i]
	}
	return shuffle
}

/**
 * Your Solution object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.Reset();
 * param_2 := obj.Shuffle();
 */

func max(n1, n2 int) int {
	if n1 > n2 {
		return n1
	}
	return n2
}

func main() {
	nums := []int{2, 7, 9, 3, 1}

	obj := Constructor(nums)
	param_2 := obj.Shuffle()

	start := time.Now()
	result := param_2
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
