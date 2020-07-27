package main

import (
	"fmt"
	"time"
)

func firstUniqChar(s string) int {
	m := make(map[int32]int)
	for _, c := range s {
		if v, ok := m[c]; ok {
			m[c] = v + 1
		} else {
			m[c] = 1
		}
	}
	for i, c := range s {
		if m[c] == 1 {
			return i
		}
	}
	return -1
}

func main() {
	s := "loveleetcode"

	var start = time.Now()
	var result = firstUniqChar(s)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
