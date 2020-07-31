package main

import (
	"fmt"
	"time"
)

func strStr(haystack string, needle string) int {
	if needle == "" {
		return 0
	}
	length := len(needle)
	for i := 0; i < len(haystack)-len(needle)+1; i++ {
		if haystack[i:i+length] == needle {
			return i
		}
	}
	return -1
}

func main() {
	var haystack = "hello"
	var needle = "ll"

	var start = time.Now()
	var result = strStr(haystack, needle)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
