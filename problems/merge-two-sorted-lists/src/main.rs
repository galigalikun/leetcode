fn main() {
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None }))
                        }))
                    }))
                }))
            }))
        }))
    );
    assert_eq!(Solution::merge_two_lists(None, None), None);
    assert_eq!(
        Solution::merge_two_lists(None, Some(Box::new(ListNode { val: 0, next: None }))),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode { val: 2, next: None })),
            Some(Box::new(ListNode { val: 1, next: None }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
    );
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode { val: 5, next: None })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
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

pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = l1;
        let mut list2 = l2;
        let mut list = Vec::new();
        loop {
            let mut i = 0;
            if let Some(l) = list1 {
                list1 = l.next;
                list.push(l.val);
                i += 1;
            }
            if let Some(l) = list2 {
                list2 = l.next;
                list.push(l.val);
                i += 1;
            }
            if i == 0 {
                break;
            }
        }
        let mut result: Option<Box<ListNode>> = None;
        list.sort_by(|a, b| b.cmp(a));
        for i in list {
            result = Some(Box::new(ListNode {
                val: i,
                next: result,
            }))
        }
        return result;
    }
}
