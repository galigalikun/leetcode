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
        let mut head = head;
        let mut len = 0;
        let mut cur = &mut head;
        while let Some(node) = cur {
            len += 1;
            cur = &mut node.next;
        }
        let mut res = vec![None; k as usize];
        let mut cur = &mut head;
        let mut i = 0;
        while let Some(node) = cur {
            let mut next = &mut node.next;
            res[i as usize] = Some(node);
            i += 1;
            if i == k {
                i = 0;
            }
            cur = next;
        }
        // for i in 0..k as usize {
        //     if res[i as usize].is_none() {
        //         res[i as usize] = None;
        //     }
        // }
        return res;
    }
}
