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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mut root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        root.borrow_mut().left = Self::sorted_array_to_bst(nums[..mid].to_vec());
        root.borrow_mut().right = Self::sorted_array_to_bst(nums[mid + 1..].to_vec());
        Some(root)
    }
}

fn main() {
    let root = Vec::from([-10, -3, 0, 5, 9]);

    let start = Instant::now();
    let result = Solution::sorted_array_to_bst(root);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
