fn main() {
    assert_eq!(
        Solution::reverse_between(
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
            2,
            4
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))
    );
    assert_eq!(
        Solution::reverse_between(Some(Box::new(ListNode { val: 5, next: None })), 1, 1),
        Some(Box::new(ListNode { val: 5, next: None }))
    );
}

pub struct Solution {}
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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut i = 1;
        let mut f = Vec::new();
        let mut r = Vec::new();
        loop {
            if let Some(h) = list {
                list = h.next;
                if n >= i && i >= m {
                    r.push(h.val);
                } else if n < i {
                    while let Some(p) = r.pop() {
                        f.push(p);
                    }
                    f.push(h.val);
                } else {
                    f.push(h.val);
                }
                i += 1;
            } else {
                break;
            }
        }
        while let Some(p) = r.pop() {
            f.push(p);
        }

        let mut result: Option<Box<ListNode>> = None;

        for p in f.into_iter().rev() {
            result = Some(Box::new(ListNode {
                val: p,
                next: result,
            }));
        }

        return result;
    }
}
