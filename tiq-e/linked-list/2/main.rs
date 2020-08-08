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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut right = dummy.clone();
        let mut left = dummy.as_mut();

        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();

        dummy.next
    }
}

fn main() {
    let head = Option::from(Box::new(ListNode { val: 1, next: None }));

    let start = Instant::now();
    let result = Solution::remove_nth_from_end(head, 1);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
