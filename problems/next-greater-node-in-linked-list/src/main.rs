fn main() {
    assert_eq!(
        Solution::next_larger_nodes(Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 5, next: None }))
            }))
        }))),
        vec![5, 5, 0]
    );
    assert_eq!(
        Solution::next_larger_nodes(Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))),
        vec![7, 0, 5, 5, 0]
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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut result = vec![];
        let mut head = head;
        while let Some(node) = head {
            while let Some(&last) = stack.last() {
                if last.0 < node.val {
                    result[last.1] = node.val;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((node.val, result.len()));
            result.push(0);
            head = node.next;
        }
        return result;
    }
}
