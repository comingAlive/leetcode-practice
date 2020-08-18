package main

import (
	"fmt"
	"strconv"
	"time"
)

func fizzBuzz(n int) []string {
	ss := []string{}
	for i := 1; i <= n; i++ {
		s := ""
		if i%3 == 0 {
			s += "Fizz"
		}
		if i%5 == 0 {
			s += "Buzz"
		}
		if s == "" {
			s = strconv.Itoa(i)
		}
		ss = append(ss, s)
	}
	return ss
}

func main() {
	n := 15

	start := time.Now()
	result := fizzBuzz(n)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
