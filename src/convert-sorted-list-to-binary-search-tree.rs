fn main() {
    assert_eq!(
        Solution::sorted_list_to_bst(Some(Box::new(ListNode {
            val: -10,
            next: Some(Box::new(ListNode {
                val: -3,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 9, next: None }))
                    }))
                }))
            }))
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))
    );

    assert_eq!(Solution::sorted_list_to_bst(None), None);

    assert_eq!(
        Solution::sorted_list_to_bst(Some(Box::new(ListNode { val: 0, next: None }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::sorted_list_to_bst(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 3, next: None }))
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: None
        })))
    );
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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(list: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if list.len() == 0 {
            return None;
        }

        return Some(Rc::new(RefCell::new(TreeNode {
            val: list[list.len() / 2],
            left: Solution::helper(list[..list.len() / 2].to_vec()),
            right: Solution::helper(list[list.len() / 2 + 1..].to_vec()),
        })));
    }
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = vec![];
        let mut m_head = head;
        loop {
            if let Some(h) = m_head {
                list.push(h.val);
                m_head = h.next;
            } else {
                break;
            }
        }

        return Solution::helper(list);
    }
}
