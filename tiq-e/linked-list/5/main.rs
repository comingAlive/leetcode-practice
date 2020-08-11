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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut rev: Option<Box<ListNode>> = None;
        let mut _nxt: Option<Box<ListNode>> = None;

        while head.is_some() {
            if rev == head || rev == head.as_ref().unwrap().next {
                return true;
            }

            _nxt = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = rev;
            rev = head;
            head = _nxt;
        }

        false
    }
}

fn main() {
    let head = Option::from(Box::new(ListNode { val: 1, next: None }));

    let start = Instant::now();
    let result = Solution::is_palindrome(head);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
