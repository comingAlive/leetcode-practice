package main

import (
	"fmt"
	"time"
)

func isAnagram(s string, t string) bool {
	data := make([]int, 26)
	for _, c := range s {
		data[c-'a']++

	}
	for _, c := range t {
		data[c-'a']--
	}

	for _, cnt := range data {
		if cnt != 0 {
			return false
		}
	}
	return true
}

func main() {
	s := "ba"
	t := "abbbbb"

	var start = time.Now()
	var result = isAnagram(s, t)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
