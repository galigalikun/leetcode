fn main() {
    assert_eq!(
        Solution::reverse_list(Some(Box::new(ListNode {
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
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::reverse_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None }))
        }))
    );

    assert_eq!(Solution::reverse_list(None), None);
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
    fn helper(result: &mut Vec<i32>, head: Option<Box<ListNode>>) {
        if let Some(h) = head {
            result.push(h.val);
            Solution::helper(result, h.next);
        }
    }
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = &mut vec![];
        Solution::helper(result, head);
        let mut list: Option<Box<ListNode>> = None;
        for r in result {
            list = Some(Box::new(ListNode {
                val: *r,
                next: list,
            }));
        }
        return list;
    }
}
