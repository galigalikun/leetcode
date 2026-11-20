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
        // Convert list to vector for easier manipulation
        let mut nodes = Vec::new();
        let mut current = head;
        
        while let Some(mut node) = current {
            current = node.next.take();
            nodes.push(node.val);
        }
        
        // Keep removing zero-sum sequences until none exist
        loop {
            let mut sum = 0;
            let mut prev_sums = std::collections::HashMap::new();
            prev_sums.insert(0, -1i32);
            
            let mut removed = false;
            for i in 0..nodes.len() {
                sum += nodes[i];
                
                if prev_sums.contains_key(&sum) {
                    // Found a zero-sum sequence
                    let start = (*prev_sums.get(&sum).unwrap() + 1) as usize;
                    nodes.drain(start..=i);
                    removed = true;
                    break;
                }
                
                prev_sums.insert(sum, i as i32);
            }
            
            if !removed {
                break;
            }
        }
        
        // Convert vector back to linked list
        let mut result = None;
        for &val in nodes.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = result;
            result = Some(node);
        }
        
        result
    }
}
