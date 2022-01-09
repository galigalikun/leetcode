fn main() {
    assert_eq!(
        Solution::is_subtree(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            })))
        ),
        true
    );

    assert_eq!(
        Solution::is_subtree(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            })))
        ),
        false
    );
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

// https://qiita.com/redpeaks33/items/8366f3a4f8b5ed894980
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        return match (root, sub_root) {
            (Some(r), Some(s)) => {
                if r.borrow().val != s.borrow().val {
                    false
                } else {
                    Solution::helper(r.borrow().left.clone(), s.borrow().left.clone())
                        & Solution::helper(r.borrow().right.clone(), s.borrow().right.clone())
                }
            }
            (Some(_r), None) => false,
            (None, Some(_s)) => false,
            (None, None) => true,
        };
    }
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        return match (root.clone(), sub_root.clone()) {
            (Some(r), Some(s)) => {
                if r.borrow().val == s.borrow().val {
                    if Solution::helper(root.clone(), sub_root.clone()) {
                        true
                    } else {
                        Solution::is_subtree(r.borrow().left.clone(), sub_root.clone())
                            || Solution::is_subtree(r.borrow().right.clone(), sub_root.clone())
                    }
                } else {
                    Solution::is_subtree(r.borrow().left.clone(), sub_root.clone())
                        || Solution::is_subtree(r.borrow().right.clone(), sub_root.clone())
                }
            }
            _ => false,
        };
    }
}
