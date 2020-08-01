package main

import (
	"fmt"
	"time"
)

func countAndSay(n int) string {
	if n == 1 {
		return "1"
	}
	if n == 2 {
		return "11"
	}
	prev := []byte{'1', '1'}
	for i := 2; i < n; i++ {
		var temp []byte
		count := 1
		for j := 1; j < len(prev); j++ {
			if prev[j] == prev[j-1] {
				count++
			} else {
				temp = append(temp, byte(count+'0'), prev[j-1])
				count = 1
			}
			if j == len(prev)-1 {
				temp = append(temp, byte(count+'0'), prev[j])
				count = 0
			}
		}
		prev = temp
	}
	return string(prev)
}

func main() {
	var n = 4

	var start = time.Now()
	var result = countAndSay(n)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
