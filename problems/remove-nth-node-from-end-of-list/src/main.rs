fn main() {
    assert_eq!(
        Solution::remove_nth_from_end(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        }))
                    }))
                }))
            })),
            2
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1),
        None
    );
    assert_eq!(
        Solution::remove_nth_from_end(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            })),
            1
        ),
        Some(Box::new(ListNode { val: 1, next: None }))
    )
}

// Definition for singly-linked list.
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
struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut list = head;
        loop {
            if let Some(h) = list {
                list = h.next;
                result = Some(Box::new(ListNode {
                    val: h.val,
                    next: result,
                }));
            } else {
                break;
            }
        }
        let mut no = 1;
        let mut rev: Option<Box<ListNode>> = None;
        loop {
            if let Some(h) = result {
                result = h.next;
                if n != no {
                    rev = Some(Box::new(ListNode {
                        val: h.val,
                        next: rev,
                    }));
                }
            } else {
                break;
            }
            no += 1;
        }
        return rev;
    }
}
