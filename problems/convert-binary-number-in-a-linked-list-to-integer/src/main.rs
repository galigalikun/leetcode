fn main() {
    assert_eq!(
        Solution::get_decimal_value(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }))),
        5
    );
    assert_eq!(
        Solution::get_decimal_value(Some(Box::new(ListNode { val: 0, next: None }))),
        0
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
    fn get_decimal_value_helper(head: Option<Box<ListNode>>, mut sum: i32) -> i32 {
        match head {
            Some(node) => {
                sum = sum * 2 + node.val;
                return Solution::get_decimal_value_helper(node.next, sum);
            }
            None => return sum,
        }
    }
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        return Solution::get_decimal_value_helper(head, 0);
    }
}
