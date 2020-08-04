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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::valid(root, std::i64::MIN, std::i64::MAX)
    }

    fn valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = node {
            if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
                return false;
            }
            return Self::valid(node.borrow().left.clone(), min, node.borrow().val as i64)
                && Self::valid(node.borrow().right.clone(), node.borrow().val as i64, max);
        }
        true
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
