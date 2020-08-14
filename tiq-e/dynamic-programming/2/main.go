package main

import (
	"fmt"
	"math"
	"time"
)

func maxProfit(prices []int) int {
	minprice := math.MaxInt32
	maxprofit := 0
	for _, price := range prices {
		if price < minprice {
			minprice = price
		} else if price-minprice > maxprofit {
			maxprofit = price - minprice
		}
	}
	return maxprofit
}

func main() {
	var n = []int{7, 1, 5, 3, 6, 4}

	var start = time.Now()
	var result = maxProfit(n)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
