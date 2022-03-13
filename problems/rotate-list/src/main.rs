fn main() {
    assert_eq!(
        Solution::rotate_right(
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
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::rotate_right(
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None }))
                }))
            })),
            4
        ),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        }))
    );
}

struct Solution {}
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
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut work = Vec::new();
        loop {
            if let Some(h) = list {
                work.push(h.val);
                list = h.next;
            } else {
                break;
            }
        }

        let mut buf = vec![0; work.len()];
        for i in 0..work.len() {
            buf[(i + k as usize) % work.len()] = work[i];
        }

        let mut result: Option<Box<ListNode>> = None;

        for &b in buf.iter().rev() {
            result = Some(Box::new(ListNode {
                val: b,
                next: result,
            }));
        }

        return result;
    }
}
