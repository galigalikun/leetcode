fn main() {
    assert_eq!(
        Solution::odd_even_list(Some(Box::new(ListNode {
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
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::odd_even_list(Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 7, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 4, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = 0;
        let mut root = head;
        let mut front = vec![];
        let mut back = vec![];
        loop {
            if let Some(h) = root {
                if i % 2 == 0 {
                    front.push(h.val);
                } else {
                    back.push(h.val);
                }
                root = h.next;
            } else {
                break;
            }
            i += 1;
        }
        front.extend(back);
        let mut result: Option<Box<ListNode>> = None;
        for v in front.iter().rev() {
            result = Some(Box::new(ListNode {
                val: *v,
                next: result,
            }));
        }
        return result;
    }
}
