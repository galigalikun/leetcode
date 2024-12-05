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
struct Solution;
impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut a = a;
        let mut b = b;
        let mut cur = &mut list1;
        while a > 1 {
            cur = &mut cur.as_mut().unwrap().next;
            a -= 1;
            b -= 1;
        }
        let mut cur2 = &mut list2;
        while b > 0 {
            cur2 = &mut cur2.as_mut().unwrap().next;
            b -= 1;
        }
        let mut cur2 = cur2.take();
        cur.as_mut().unwrap().next = list2;
        list1
    }
}
fn main() {
    assert_eq!(
        Solution::merge_in_between(
            Some(Box::new(ListNode {
                val: 10,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 13,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            }))
                        }))
                    }))
                })),
            })),
            3,
            4,
            Some(Box::new(ListNode {
                val: 1000000,
                next: Some(Box::new(ListNode {
                    val: 1000001,
                    next: Some(Box::new(ListNode {
                        val: 1000002,
                        next: None
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 10,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 13,
                    next: Some(Box::new(ListNode {
                        val: 1000000,
                        next: Some(Box::new(ListNode {
                            val: 1000001,
                            next: Some(Box::new(ListNode {
                                val: 1000002,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
    assert_eq!(
        Solution::merge_in_between(
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            2,
            5,
            Some(Box::new(ListNode {
                val: 1000000,
                next: Some(Box::new(ListNode {
                    val: 1000001,
                    next: Some(Box::new(ListNode {
                        val: 1000002,
                        next: Some(Box::new(ListNode {
                            val: 1000003,
                            next: Some(Box::new(ListNode {
                                val: 1000004,
                                next: None
                            }))
                        }))
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1000000,
                    next: Some(Box::new(ListNode {
                        val: 1000001,
                        next: Some(Box::new(ListNode {
                            val: 1000002,
                            next: Some(Box::new(ListNode {
                                val: 1000003,
                                next: Some(Box::new(ListNode {
                                    val: 1000004,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
}
