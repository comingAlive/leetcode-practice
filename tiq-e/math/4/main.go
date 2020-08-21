package main

import (
	"fmt"
	"time"
)

func romanToInt(s string) int {
	romanValue := map[rune]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	var arabic int

	var inputLength = len(s)
	for i, char := range s {
		if i+1 < inputLength && romanValue[char] < romanValue[rune(s[i+1])] {
			arabic -= romanValue[char]
		} else {
			arabic += romanValue[char]
		}
	}

	return arabic
}

func main() {
	n := "M"

	start := time.Now()
	result := romanToInt(n)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
