use std::time::Instant;

struct Solution;

struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.min.len() == 0 || x <= self.get_min() {
            self.min.push(x);
        }
        self.stack.push(x);
    }

    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.min.pop();
        }
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {
    let start = Instant::now();
    let mut result = MinStack::new();

    result.push(1);
    result.push(6);
    result.push(3);
    result.top();
    result.get_min();
    result.pop();

    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result.stack);
}
