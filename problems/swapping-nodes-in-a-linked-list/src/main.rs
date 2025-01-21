fn main() {
    assert_eq!(
        Solution::swap_nodes(
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
        Solution::swap_nodes(
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 7,
                                next: Some(Box::new(ListNode {
                                    val: 8,
                                    next: Some(Box::new(ListNode {
                                        val: 3,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 5,
                                                    next: None
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            5
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 8,
                            next: Some(Box::new(ListNode {
                                val: 7,
                                next: Some(Box::new(ListNode {
                                    val: 3,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode { val: 5, next: None }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
}

struct Solution;
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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut ans = vec![];
        let mut head = head;
        while let Some(node) = head {
            ans.push(node.val);
            head = node.next;
        }
        let k = k as usize;
        let n = ans.len();
        ans.swap(k - 1, n - k);
        let mut head = None;
        for &val in ans.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
