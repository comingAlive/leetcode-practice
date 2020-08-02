package main

import (
	"fmt"
	"time"
)

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}

	var (
		lenPrefix int
		curChar   byte
	)

	for {
		if lenPrefix >= len(strs[0]) {
			break
		}

		curChar = strs[0][lenPrefix]

		for _, w := range strs {
			if lenPrefix >= len(w) || curChar != w[lenPrefix] {
				return w[:lenPrefix]
			}
		}

		lenPrefix += 1
	}
	return strs[0][:lenPrefix]
}

func main() {
	var arr = []string{"flower", "flow", "flight"}

	var start = time.Now()
	var result = longestCommonPrefix(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
