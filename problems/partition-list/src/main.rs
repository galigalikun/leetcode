fn main() {
    assert_eq!(
        Solution::partition(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 2, next: None }))
                            }))
                        }))
                    }))
                }))
            })),
            3
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        }))
                    }))
                }))
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut p0 = Vec::new();
        let mut p1 = Vec::new();
        loop {
            if let Some(h) = list {
                if x > h.val {
                    p0.push(h.val);
                } else {
                    p1.push(h.val);
                }
                list = h.next;
            } else {
                break;
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        p0.append(&mut p1);
        for i in p0.into_iter().rev() {
            result = Some(Box::new(ListNode {
                val: i,
                next: result,
            }));
        }
        return result;
    }
}
