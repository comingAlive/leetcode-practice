package main

import (
	"fmt"
	"time"
)

func isValidSudoku(board [][]byte) bool {
	vrow := make(map[int]map[byte]bool)
	vcol := make(map[int]map[byte]bool)
	for box := 0; box < len(board); box++ {
		vcube := make(map[byte]bool)
		for i := box / 3 * 3; i < box/3*3+3; i++ {
			for j := box % 3 * 3; j < box%3*3+3; j++ {
				val := board[i][j]
				if val == '.' {
					continue
				}
				if vcube[val] || vrow[i][val] || vcol[j][val] {
					return false
				}
				if vrow[i] == nil {
					vrow[i] = make(map[byte]bool)
				}
				if vcol[j] == nil {
					vcol[j] = make(map[byte]bool)
				}
				vrow[i][val], vcol[j][val], vcube[val] = true, true, true
			}
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
