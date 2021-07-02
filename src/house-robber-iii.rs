fn main() {
    assert_eq!(
        Solution::rob(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            })))
        })))),
        7
    );
}

pub struct Solution {}
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
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let left = Solution::helper(r.borrow().left.clone());
            let right = Solution::helper(r.borrow().right.clone());
            return (r.borrow().val + (left >> 32) + (right >> 32)) | (std::cmp::max(left >> 32, left & 0xffff_ffff as i64) + std::cmp::max(right >> 32, right & 0xffff_ffff as i64)) << 32;
        }

        return 0;
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return 0;
    }
}
