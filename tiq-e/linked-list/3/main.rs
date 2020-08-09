use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(c) = cur {
            prev = Some(Box::new(ListNode {
                val: c.val,
                next: prev,
            }));
            cur = c.next;
        }

        prev
    }
}

fn main() {
    let head = Option::from(Box::new(ListNode { val: 1, next: None }));

    let start = Instant::now();
    let result = Solution::reverse_list(head);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
