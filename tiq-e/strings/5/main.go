package main

import (
	"fmt"
	"regexp"
	"strings"
	"time"
)

func isPalindrome(s string) bool {
	var reg, _ = regexp.Compile("[^a-z0-9]+")
	s = reg.ReplaceAllString(strings.ToLower(s), "")

	c1, c2 := 0, len(s)-1

	for c1 < c2 {
		if s[c1] != s[c2] {
			return false
		}
		c1++
		c2--
	}
	return true
}

func main() {
	s := "aba"

	var start = time.Now()
	var result = isPalindrome(s)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
