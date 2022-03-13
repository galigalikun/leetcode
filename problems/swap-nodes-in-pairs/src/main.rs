fn main() {
    assert_eq!(
        Solution::swap_pairs(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        }))
    );
    assert_eq!(Solution::swap_pairs(None), None);
    assert_eq!(
        Solution::swap_pairs(Some(Box::new(ListNode { val: 1, next: None }))),
        Some(Box::new(ListNode { val: 1, next: None }))
    );
    assert_eq!(
        Solution::swap_pairs(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))
    );
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut buf1 = Vec::new();
        let mut buf2 = Vec::new();
        let mut i = 0;
        let mut list = head;
        loop {
            if let Some(h) = list {
                list = h.next;
                if i % 2 == 0 {
                    buf1.push(h.val);
                } else {
                    buf2.push(h.val);
                }
            } else {
                break;
            }
            i += 1;
        }

        let mut result: Option<Box<ListNode>> = None;
        for i in 0..buf1.len() {
            if buf2.len() > i {
                result = Some(Box::new(ListNode {
                    val: buf2[i],
                    next: result,
                }));
            }
            result = Some(Box::new(ListNode {
                val: buf1[i],
                next: result,
            }));
        }
        let mut rev: Option<Box<ListNode>> = None;
        loop {
            if let Some(r) = result {
                rev = Some(Box::new(ListNode {
                    val: r.val,
                    next: rev,
                }));
                result = r.next;
            } else {
                break;
            }
        }
        return rev;
    }
}
