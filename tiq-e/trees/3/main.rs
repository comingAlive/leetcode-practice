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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => Solution::is_mirror(&r.borrow().left, &r.borrow().right),
            None => true,
        }
    }

    fn is_mirror(
        tree1: &Option<Rc<RefCell<TreeNode>>>,
        tree2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (tree1, tree2) {
            (None, None) => true,
            (Some(tree1), Some(tree2)) => {
                tree1.borrow().val == tree2.borrow().val
                    && Solution::is_mirror(&tree1.borrow().left, &tree2.borrow().right)
                    && Solution::is_mirror(&tree1.borrow().right, &tree2.borrow().left)
            }
            _ => false,
        }
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
