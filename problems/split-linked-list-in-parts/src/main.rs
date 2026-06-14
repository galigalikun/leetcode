fn main() {
    assert_eq!(
        Solution::split_list_to_parts(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            5
        ),
        vec![
            Some(Box::new(ListNode { val: 1, next: None })),
            Some(Box::new(ListNode { val: 2, next: None })),
            Some(Box::new(ListNode { val: 3, next: None })),
            None,
            None,
        ]
    );

    assert_eq!(
        Solution::split_list_to_parts(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: Some(Box::new(ListNode {
                                        val: 7,
                                        next: Some(Box::new(ListNode {
                                            val: 8,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 10,
                                                    next: None,
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                }))
            })),
            3
        ),
        vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 7, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 10,
                        next: None
                    })),
                })),
            })),
        ]
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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;

        let mut len = 0;
        let mut node = head.as_ref();
        while let Some(n) = node {
            len += 1;
            node = n.next.as_ref();
        }

        let base_size = len / k;
        let extra = len % k;

        let mut result = Vec::with_capacity(k);
        let mut current = head;

        for i in 0..k {
            let part_size = base_size + usize::from(i < extra);

            if part_size == 0 {
                result.push(None);
                continue;
            }

            let mut part_head = current.take();
            let mut tail = part_head.as_mut();

            for _ in 1..part_size {
                if let Some(node) = tail {
                    tail = node.next.as_mut();
                }
            }

            if let Some(node) = tail {
                current = node.next.take();
            }

            result.push(part_head);
        }

        result
    }
}
