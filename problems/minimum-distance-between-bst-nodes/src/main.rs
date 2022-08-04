fn main() {
    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })))),
        1
    );
    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 48,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 49,
                    left: None,
                    right: None,
                }))),
            }))),
        })))),
        1
    );

    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 96,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 12,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 52,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 29,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
            }))),
            right: None,
        })))),
        1
    );

    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 90,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 69,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 49,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 52,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 89,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        })))),
        1
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
impl Solution {
    fn helper(result: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if let Some(left) = r.borrow().left.clone() {
                *result = std::cmp::min(*result, (r.borrow().val - left.borrow().val).abs());
            }
            if let Some(right) = r.borrow().right.clone() {
                *result = std::cmp::min(*result, (r.borrow().val - right.borrow().val).abs());
            }
            Solution::helper(result, r.borrow().left.clone());
            Solution::helper(result, r.borrow().right.clone());
        }
    }
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = std::i32::MAX;
        Solution::helper(&mut ans, root);
        return ans;
    }
}
