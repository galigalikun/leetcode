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

    assert_eq!(
        Solution::rob(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            })))
        })))),
        9
    );
}

struct Solution {}
// https://zenn.dev/komiya_atsushi/articles/f51c3573642e75b73a42
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
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
        if let Some(r) = root {
            let left = Solution::helper(r.borrow().left.clone());
            let right = Solution::helper(r.borrow().right.clone());
            return (r.borrow().val as i64 + (left as i64 >> 32) + (right as i64 >> 32))
                | (std::cmp::max(left as i64 >> 32, left as i64 & 0xffffffff)
                    + std::cmp::max(right as i64 >> 32, right as i64 & 0xffffffff))
                    << 32;
        }

        return 0;
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Solution::helper(root);
        return std::cmp::max(result >> 32, result & 0xffffffff) as i32;
    }
}
