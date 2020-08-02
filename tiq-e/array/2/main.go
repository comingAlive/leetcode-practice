package main

import (
	"fmt"
	"time"
)

func maxProfit(prices []int) int {
	profit := 0

	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			profit += prices[i] - prices[i-1]
		}
	}

	return profit
}

func main() {
	var prices = []int{7, 1, 5, 3, 6, 4}

	var start = time.Now()
	var result = maxProfit(prices)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
