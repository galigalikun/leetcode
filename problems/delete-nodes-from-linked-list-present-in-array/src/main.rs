fn main() {
    assert_eq!(
        Solution::modified_list(
            vec![1, 2, 3],
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5)))
                        }))
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(5))),
        }))
    );
    assert_eq!(
        Solution::modified_list(
            vec![2, 4, 6, 8, 10],
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode::new(2)))
                            }))
                        }))
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(6)))
            }))
        }))
    );
    assert_eq!(Solution::modified_list(vec![1, 3, 5, 7, 9], None), None);
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
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = head;
        let mut current = new_head.as_mut();
        let mut idx = 0;

        while let Some(node) = current {
            if idx % 2 == 0 {
                if let Some(val) = nums.get(idx) {
                    node.val = *val;
                }
            }
            current = node.next.as_mut();
            idx += 1;
        }

        new_head
    }
}
