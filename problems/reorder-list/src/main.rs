fn main() {
    let mut a = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    Solution::reorder_list(&mut a);
    assert_eq!(
        a,
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        }))
    );

    a = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    Solution::reorder_list(&mut a);

    assert_eq!(
        a,
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut list = head.as_ref();
        let mut work = Vec::new();
        loop {
            if let Some(l) = list {
                work.push(l.val);
                list = l.next.as_ref();
            } else {
                break;
            }
        }

        let max = work.len() as usize;
        let mut tmp = Vec::new();
        for i in 0..max {
            if (i + 1) % 2 == 0 {
                tmp.push(work[max - (i + 1) / 2]);
            } else {
                tmp.push(work[i / 2]);
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        for &v in tmp.iter().rev() {
            result = Some(Box::new(ListNode {
                val: v,
                next: result,
            }));
        }

        *head = result;
    }
}
