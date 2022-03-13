fn main() {
    assert_eq!(
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        true
    );

    assert_eq!(
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                })))
            })))
        })))),
        false
    );

    assert_eq!(
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: -2147483648,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2147483648,
                left: None,
                right: None
            }))),
            right: None
        })))),
        false
    );
}

struct Solution {}
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
// https://www.geeksforgeeks.org/a-program-to-check-if-a-binary-tree-is-bst-or-not/
impl Solution {
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(r) = root {
            if (r.borrow().val as i64) < min || (r.borrow().val as i64) > max {
                return false;
            }
            return Solution::helper(r.borrow().left.clone(), min, r.borrow().val as i64 - 1)
                && Solution::helper(r.borrow().right.clone(), r.borrow().val as i64 + 1, max);
        }
        return true;
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::helper(root, std::i32::MIN as i64, std::i32::MAX as i64);
    }
}
