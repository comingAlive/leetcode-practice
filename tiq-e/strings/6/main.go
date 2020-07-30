package main

import (
	"fmt"
	"math"
	"time"
)

func myAtoi(str string) int {
	sign, ret, i := 1, 0, 0

	for ; i < len(str) && str[i] == ' '; i++ {
	}

	if i < len(str) && (str[i] == '+' || str[i] == '-') {
		if str[i] == '-' {
			sign = -1
		}
		i++
	}
	for ; i < len(str) && '0' <= str[i] && str[i] <= '9'; i++ {
		ret = ret*10 + int(str[i]-'0')
		if sign*ret < math.MinInt32 {
			return math.MinInt32
		}
		if sign*ret > math.MaxInt32 {
			return math.MaxInt32
		}
	}
	return sign * ret
}

func main() {
	s := "42"

	var start = time.Now()
	var result = myAtoi(s)
	var end = time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result)
}
