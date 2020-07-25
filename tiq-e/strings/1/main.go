package main

import (
	"fmt"
	"time"
)

func reverseString(s []byte) {
	half, max := len(s)>>1, len(s)-1
	for i := 0; i < half; i++ {
		s[i], s[max-i] = s[max-i], s[i]
	}
}

func main() {
	arr := []byte{'h', 'e', 'l', 'l', 'o'}

	var start = time.Now()
	reverseString(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", arr)
}
