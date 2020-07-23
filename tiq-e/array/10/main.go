package main

import (
	"fmt"
	"time"
)

func isValidSudoku(board [][]byte) bool {
	var row, col, block [300][300]bool
	for i := range board {
		for j, c := range board[i] {
			if c == '.' {
				continue
			}
			if col[j][c-'0'] || row[i][c-'0'] || block[i/3+j/3*3][c-'0'] {
				return false
			}
			col[j][c-'0'] = true
			row[i][c-'0'] = true
			block[i/3+j/3*3][c-'0'] = true
		}
	}
	return true
}

func main() {
	arr := [][]byte{
		{5, 3, '.', '.', 7, '.', '.', '.', '.'},
		{6, '.', '.', 1, 9, 5, '.', '.', '.'},
		{'.', 9, 8, '.', '.', '.', '.', 6, '.'},
		{8, '.', '.', '.', 6, '.', '.', '.', 3},
		{4, '.', '.', 8, '.', 3, '.', '.', 1},
		{7, '.', '.', '.', 2, '.', '.', '.', 6},
		{'.', 6, '.', '.', '.', '.', 2, 8, '.'},
		{'.', '.', '.', 4, 1, 9, '.', '.', 5},
		{'.', '.', '.', '.', 8, '.', '.', 7, 9},
	}

	var start = time.Now()
	var result = isValidSudoku(arr)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
