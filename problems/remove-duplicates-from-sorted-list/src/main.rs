fn main() {
    assert_eq!(
        Solution::delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
    );
    assert_eq!(
        Solution::delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 3, next: None }))
                        }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut prev: Option<i32> = None;
        let mut work = Vec::new();
        loop {
            if let Some(h) = list {
                if prev != Some(h.val) {
                    work.push(h.val);
                }

                prev = Some(h.val);
                list = h.next;
            } else {
                break;
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        for w in work.into_iter().rev() {
            result = Some(Box::new(ListNode {
                val: w,
                next: result,
            }));
        }

        return result;
    }
}
