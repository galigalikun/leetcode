fn main() {
    assert_eq!(
        Solution::sort_list(Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }))
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut work = Vec::new();
        loop {
            if let Some(h) = list {
                work.push(h.val);
                list = h.next;
            } else {
                break;
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        work.sort();
        for i in work.into_iter().rev() {
            result = Some(Box::new(ListNode {
                val: i,
                next: result,
            }));
        }

        return result;
    }
}
