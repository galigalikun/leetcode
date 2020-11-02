fn main() {
    assert_eq!(
        Solution::reverse_k_group(
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
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::reverse_k_group(
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
            3
        ),
        Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::reverse_k_group(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            })),
            2
        ),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None }))
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
pub struct Solution {}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut buf: Option<Box<ListNode>> = head;
        let mut work: Vec<i32> = Vec::new();
        let mut list: Vec<i32> = Vec::new();

        loop {
            if let Some(h) = buf {
                if work.len() == k as usize {
                    for &j in work.iter().rev() {
                        list.push(j);
                    }
                    work.clear();
                }
                work.push(h.val);
                buf = h.next;
            } else {
                break;
            }
        }

        if work.len() == k as usize {
            for &j in work.iter().rev() {
                list.push(j);
            }
        } else {
            for j in work {
                list.push(j);
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        for &i in list.iter().rev() {
            result = Some(Box::new(ListNode {
                val: i,
                next: result,
            }));
        }

        return result;
    }
}
