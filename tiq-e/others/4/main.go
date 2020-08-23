package main

import (
	"fmt"
	"time"
)

func generate(numRows int) [][]int {
	var answer [][]int
	if numRows == 0 {
		return answer
	}
	answer = append(answer, []int{1})
	for i := 1; i < numRows; i++ {

		row := make([]int, i+1)
		row[0] = 1
		row[len(row)-1] = 1

		for j := 1; j < len(row)-1; j++ {
			row[j] = answer[i-1][j-1] + answer[i-1][j]
		}
		answer = append(answer, row)
	}
	return answer
}

func main() {
	x := 5

	start := time.Now()
	result := generate(x)
	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
