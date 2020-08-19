package main

import (
	"fmt"
	"strconv"
	"time"
)

func fizzBuzz(n int) []string {
	arr := make([]string, n)
	for i := 0; i < len(arr); i++ {
		val := i + 1
		strVal := strconv.Itoa(val)
		if val%15 == 0 {
			strVal = "FizzBuzz"
		} else if val%3 == 0 {
			strVal = "Fizz"
		} else if val%5 == 0 {
			strVal = "Buzz"
		}
		arr[i] = strVal
	}
	return arr
}

func main() {
	n := 15

	start := time.Now()
	result := fizzBuzz(n)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
