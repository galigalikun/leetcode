fn main() {
    assert_eq!(
        Solution::remove_zero_sum_sublists(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 1, next: None }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 1, next: None }))
        }))
    );

    assert_eq!(
        Solution::remove_zero_sum_sublists(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: -3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None }))
            }))
        }))
    );

    assert_eq!(
        Solution::remove_zero_sum_sublists(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: -3,
                        next: Some(Box::new(ListNode {
                            val: -2,
                            next: None
                        }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode { val: 1, next: None }))
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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut dummy_ref = dummy.as_mut();
        dummy_ref.unwrap().next = head;
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        while let Some(node) = dummy_ref.unwrap().next.take() {
            sum += node.val;
            if let Some(n) = map.get(&sum) {
                dummy_ref = n.next.as_mut();
                sum = sum - node.val;
                while let Some(node) = dummy_ref.unwrap().next.take() {
                    sum += node.val;
                    if sum == 0 {
                        dummy_ref.unwrap().next = node.next;
                        break;
                    }
                    dummy_ref = dummy_ref.unwrap().next.as_mut();
                }
            } else {
                map.insert(sum, dummy_ref.unwrap());
                dummy_ref.unwrap().next = Some(node);
                dummy_ref = dummy_ref.unwrap().next.as_mut();
            }
        }
        return dummy.unwrap().next;
    }
}
