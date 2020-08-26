import {performance} from "perf_hooks";

class MinStack {
    stack: number[];
    min: number[];

    constructor() {
        this.stack = [];
        this.min = [];
    }

    push(x: number) {
        if (!this.min.length) this.min.push(x);
        else this.min.push(Math.min(x, this.getMin()));
        this.stack.push(x);
    }

    pop() {
        this.min.pop();
        return this.stack.pop();
    }

    top() {
        return this.stack[this.stack.length - 1];
    }

    getMin() {
        return this.min[this.min.length - 1];
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * var obj = new MinStack()
 * obj.push(x)
 * obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.getMin()
 */

const start = performance.now();
const result = new MinStack();

result.push(1);
result.push(6);
result.push(3);
result.top();
result.getMin();
result.pop();

const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result.stack);
