fn main() {
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))
        }))),
        true
    );
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))),
        false
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
    fn helper(result: &mut Vec<i32>, head: Option<Box<ListNode>>) {
        if let Some(h) = head {
            result.push(h.val);
            Solution::helper(result, h.next);
        }
    }
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut result: Vec<i32> = vec![];
        Solution::helper(&mut result, head);
        if result == result.clone().into_iter().rev().collect::<Vec<_>>() {
            return true;
        }
        return false;
    }
}
