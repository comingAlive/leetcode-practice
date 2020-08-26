package main

import (
	"fmt"
	"time"
)

type MinStack struct {
	stack []int
	min   []int
}

/** initialize your data structure here. */
func Constructor() MinStack {
	return MinStack{
		stack: []int{},
		min:   []int{},
	}
}

func (this *MinStack) Push(x int) {
	if len(this.min) == 0 || x <= this.GetMin() {
		this.min = append(this.min, x)
	}
	this.stack = append(this.stack, x)
}

func (this *MinStack) Pop() {
	if this.stack[len(this.stack)-1] == this.min[len(this.min)-1] {
		this.min = this.min[:len(this.min)-1]
	}
	this.stack = this.stack[:len(this.stack)-1]

}

func (this *MinStack) Top() int {
	return this.stack[len(this.stack)-1]
}

func (this *MinStack) GetMin() int {
	return this.min[len(this.min)-1]
}

/**
 * Your MinStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.GetMin();
 */

func main() {
	start := time.Now()
	result := MinStack{}

	result.Push(1)
	result.Push(6)
	result.Push(3)
	result.Top()
	result.GetMin()
	result.Pop()

	end := time.Since(start)
	fmt.Println("Duration: ", end)
	fmt.Println("Result: ", result.stack)
}
