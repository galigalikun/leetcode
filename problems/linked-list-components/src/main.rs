use std::collections::HashSet;

fn main() {
    assert_eq!(
        Solution::num_components(
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            })),
            vec![0, 1, 3]
        ),
        2
    );

    assert_eq!(
        Solution::num_components(
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 4, next: None }))
                        }))
                    }))
                }))
            })),
            vec![0, 3, 1, 4]
        ),
        2
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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let values: HashSet<i32> = nums.into_iter().collect();
        let mut components = 0;
        let mut in_component = false;
        let mut current = head.as_ref();

        while let Some(node) = current {
            let contains = values.contains(&node.val);
            if contains && !in_component {
                components += 1;
            }

            in_component = contains;
            current = node.next.as_ref();
        }

        components
    }
}
