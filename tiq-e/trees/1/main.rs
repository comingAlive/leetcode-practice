use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return match root {
            Some(a) => {
                let left_depth = Solution::max_depth(a.borrow().left.clone());
                let right_depth = Solution::max_depth(a.borrow().right.clone());
                return if left_depth > right_depth {
                    left_depth + 1
                } else {
                    right_depth + 1
                };
            }
            None => 0,
        };
    }
}

fn main() {
    let root = TreeNode(5, None, None);

    let start = Instant::now();
    let result = Solution::max_depth(root);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
